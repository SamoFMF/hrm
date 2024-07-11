use regex::Regex;

use crate::{
    code::{
        commands::{AnyCommand, CommandValue},
        program::{Program, ProgramBuilder},
    },
    commands,
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    IllegalLine(String),
}

#[derive(Debug)]
pub enum ParsedLine {
    Comment(u32),
    Label(String),
    CommandNew(AnyCommand),
    Empty,
    CommentedCode,
    Define(DefineInstruction),
}

#[derive(Debug, PartialEq)]
pub enum DefineInstruction {
    COMMENT(u32),
    LABEL(u32),
}

pub struct Compiler {
    pub commands: Vec<fn(&str, &str) -> Option<AnyCommand>>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            commands: commands!(),
        }
    }
}

impl Compiler {
    /// Compile HRM code consisting of instructions (e.g. [Command]) separated by new lines.
    /// Returns:
    /// - [Ok(Program)] if code was successfully parsed
    /// - [Err(ParseError)] else
    pub fn compile(&self, code: &str) -> Result<Program, ParseError> {
        let mut builder = ProgramBuilder::new();

        for line in code.lines() {
            match self.compile_instruction_new(line)? {
                ParsedLine::Label(label) => builder.add_label_ref(label),
                ParsedLine::CommandNew(command) => builder.add_command_ref_new(command),
                _ => {}
            }
        }

        Ok(builder.build())
    }

    fn compile_instruction_new(&self, instruction: &str) -> Result<ParsedLine, ParseError> {
        let instruction = instruction.trim();

        if instruction == "" {
            return Ok(ParsedLine::Empty);
        }

        if instruction.starts_with("--") && instruction.ends_with("--") {
            return Ok(ParsedLine::CommentedCode);
        }

        if let Some(id) = try_compile_comment(instruction) {
            return Ok(ParsedLine::Comment(id));
        }

        if let Some(define_instruction) = try_compile_define(instruction) {
            return Ok(ParsedLine::Define(define_instruction));
        }

        if let Some(label) = try_compile_new_label(instruction) {
            return Ok(ParsedLine::Label(label));
        }

        if let Some(command) = self.try_compile_command(instruction) {
            return Ok(ParsedLine::CommandNew(command));
        }

        Err(ParseError::IllegalLine(instruction.to_string()))
    }

    /// Tries to compile an instruction as a command. Returns:
    /// - [Ok(AnyCommand)] if instruction is a valid command with correct args
    /// - [None] else
    ///
    /// Expects instruction to be trimmed.
    fn try_compile_command(&self, instruction: &str) -> Option<AnyCommand> {
        // todo: JUMPa is accepted - assert whitespace between command & arg
        let regex = Regex::new(r"^([A-Z]+)\s*(.*)$").unwrap();
        if let Some(captures) = regex.captures(instruction) {
            let (_, [command, args]) = captures.extract();
            return self
                .commands
                .iter()
                .map(|cmd_create| cmd_create(command, args))
                .filter(|cmd_opt| cmd_opt.is_some())
                .map(|cmd_opt| cmd_opt.unwrap())
                .next();
        }

        None
    }
}

/// Tries to compile an instruction as a comment. Returns:
/// - [Ok(u32)] if line starts with <code>COMMENT</code> and has an [u32] arg
/// - [None] else
///
/// Expects instruction to be trimmed.
fn try_compile_comment(instruction: &str) -> Option<u32> {
    let regex = Regex::new(r"^COMMENT\s+(\d+)$").unwrap();
    if let Some(captures) = regex.captures(instruction) {
        let (_, [arg]) = captures.extract();
        return Some(arg.parse().unwrap());
    }

    None
}

/// Tries to compile a define instruction. Returns:
/// - [Ok(DefineLine)] if define contains the correct type & index
/// - [None] else
///
/// Expects instruction to be trimmed.
fn try_compile_define(instruction: &str) -> Option<DefineInstruction> {
    let regex = Regex::new(r"^DEFINE\s+(COMMENT|LABEL)\s+(\d+)$").unwrap();
    if let Some(captures) = regex.captures(instruction) {
        let (_, [define_type, index]) = captures.extract();
        let index = index.parse().unwrap();
        return match define_type {
            "COMMENT" => Some(DefineInstruction::COMMENT(index)),
            "LABEL" => Some(DefineInstruction::LABEL(index)),
            &_ => panic!("This cannot occur!"),
        };
    }

    None
}

/// Tries to compile an instruction as a new label. Returns:
/// - [Ok(String)] if instruction consists of lowercase a-z followed by a colon
/// - [None] else
///
/// Expects instruction to be trimmed.
fn try_compile_new_label(instruction: &str) -> Option<String> {
    let regex = Regex::new(r"^([a-z]+):$").unwrap();
    if let Some(captures) = regex.captures(instruction) {
        let (_, [label]) = captures.extract();
        return Some(label.to_string());
    }

    None
}

/// Returns [Ok(Value)] if input matches one of:
/// - <code>\d+</code>
/// - <code>\[\d+\]</code>
///
/// Returns [None] otherwise.
pub fn try_compile_command_value(value: &str) -> Option<CommandValue> {
    let regex = Regex::new(r"^(\[\d+]|\d+)$").unwrap();
    if let Some(captures) = regex.captures(value) {
        let (_, [value]) = captures.extract();
        return if value.starts_with("[") {
            let value = (&value[1..(value.len() - 1)]).parse().unwrap();
            Some(CommandValue::Index(value))
        } else {
            let value = value.parse().unwrap();
            Some(CommandValue::Value(value))
        };
    }

    None
}

/// Returns [Ok(String)] if input matches <code>\[a-z\]+</code>, else returns [None].
pub fn try_compile_label(label: &str) -> Option<String> {
    let regex = Regex::new(r"^([a-z]+)$").unwrap();
    if let Some(captures) = regex.captures(label) {
        let (_, [label]) = captures.extract();
        return Some(label.to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_compile_comment_succeeds() {
        let line = "COMMENT 123";
        let comment = try_compile_comment(line).unwrap();
        assert_eq!(123, comment);
    }

    #[test]
    fn try_compile_comment_fails() {
        for arg in vec!["", "1a", "b", "C", "aBc", "0 1"] {
            let line = format!("COMMENT {}", arg);
            let comment = try_compile_comment(&line);
            assert!(comment.is_none());
        }
    }

    #[test]
    fn try_compile_define_succeeds() {
        let index: u32 = 123;
        let define_pairs = [
            ("COMMENT", DefineInstruction::COMMENT(index)),
            ("LABEL", DefineInstruction::LABEL(index)),
        ];

        for define_pair in define_pairs {
            let line = format!("DEFINE {} {}", define_pair.0, index);
            let define_line = try_compile_define(&line).unwrap();
            assert_eq!(define_pair.1, define_line);
        }
    }

    #[test]
    fn try_compile_new_label_succeeds() {
        for line in ["a:", "abc:"] {
            let label = try_compile_new_label(line).unwrap();
            assert_eq!(&line[..line.len() - 1], label);
        }
    }

    #[test]
    fn try_compile_new_label_fails() {
        for line in ["INBOX", "A:", "aBc:", "a:b", "a: "] {
            let label = try_compile_new_label(line);
            assert!(label.is_none());
        }
    }

    #[test]
    fn try_compile_command_no_arg_succeeds() {
        let compiler = Compiler::default();

        for cmd in ["INBOX", "OUTBOX"] {
            let command = compiler.try_compile_command(cmd).unwrap();
            assert_eq!(cmd, command.command());
        }
    }

    #[test]
    fn try_compile_command_no_arg_fails() {
        let compiler = Compiler::default();

        for cmd in ["INBOX", "OUTBOX"] {
            for arg in ["1", "a", "42b"] {
                let line = format!("{} {}", cmd, arg);
                let command = compiler.try_compile_command(&line);
                assert!(command.is_none());
            }
        }
    }

    #[test]
    fn try_compile_command_value_arg_succeeds() {
        let value = 123;
        let index = 456;
        let compiler = Compiler::default();

        for cmd in ["COPYFROM", "COPYTO", "ADD", "SUB", "BUMPUP", "BUMPDN"] {
            let line = format!("{} {}", cmd, value);
            let command = compiler.try_compile_command(&line).unwrap();
            assert_eq!(cmd, command.command());
            assert_command_value(&command, CommandValue::Value(value));

            let line = format!("{} [{}]", cmd, index);
            let command = compiler.try_compile_command(&line).unwrap();
            assert_eq!(cmd, command.command());
            assert_command_value(&command, CommandValue::Index(index));
        }
    }

    #[test]
    fn try_compile_command_value_arg_fails() {
        let compiler = Compiler::default();

        for cmd in ["COPYFROM", "COPYTO", "ADD", "SUB", "BUMPUP", "BUMPDN"] {
            for arg in ["", "1a", "abc", "D", "[", "[]", "[1a]", "[A]"] {
                let line = format!("{} {}", cmd, arg);
                let command = compiler.try_compile_command(&line);
                assert!(command.is_none());
            }
        }
    }

    #[test]
    fn try_compile_command_label_arg_succeeds() {
        let label = "abc";
        let compiler = Compiler::default();

        for cmd in ["JUMP", "JUMPZ", "JUMPN"] {
            let line = format!("{} {}", cmd, label);
            let command = compiler.try_compile_command(&line).unwrap();
            assert_eq!(cmd, command.command());
            assert_label(&command, label);
        }
    }

    #[test]
    fn try_compile_command_label_arg_fails() {
        let compiler = Compiler::default();

        for cmd in ["JUMP", "JUMPZ", "JUMPN"] {
            for arg in ["", "aBc", "A", "1"] {
                let line = format!("{} {}", cmd, arg);
                let command = compiler.try_compile_command(&line);
                assert!(command.is_none());
            }
        }
    }

    #[test]
    fn try_compile_value_empty() {
        let value = try_compile_command_value("");
        assert!(value.is_none());
    }

    #[test]
    fn try_compile_value_value() {
        let value = try_compile_command_value("123").unwrap();
        assert_eq!(CommandValue::Value(123), value);
    }

    #[test]
    fn try_compile_value_index() {
        let value = try_compile_command_value("[123]").unwrap();
        assert_eq!(CommandValue::Index(123), value);
    }

    #[test]
    fn try_compile_label_succeeds() {
        for label in vec!["a", "bc", "def"] {
            let parsed_label = try_compile_label(label).unwrap();
            assert_eq!(label, parsed_label);
        }
    }

    #[test]
    fn try_compile_label_fails() {
        for label in vec!["A", "aBc", "1", "a1", "ab:", ""] {
            let label = try_compile_label(label);
            assert!(label.is_none());
        }
    }

    // region:test-utils
    fn assert_command_value(command: &AnyCommand, value: CommandValue) {
        let command = format!("{:?}", command);
        let value = format!("{:?}", value);
        assert!(command.contains(&value));
    }

    fn assert_label(command: &AnyCommand, label: &str) {
        let command = format!("{:?}", command);
        assert!(command.contains(label));
    }
    // endregion
}
