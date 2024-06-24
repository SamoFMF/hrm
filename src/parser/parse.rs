use regex::Regex;

use crate::code::commands::{Command, Value};
use crate::code::program::{Program, ProgramBuilder};
use crate::parser::parse::ParseError::IllegalLine;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    IllegalLine(String),
}

#[derive(Debug)]
pub enum ParsedLine {
    Comment(u32),
    Label(String),
    Command(Command),
    Empty,
    CommentedCode,
    Define(DefineLine),
}

#[derive(Debug)]
pub enum DefineLine {
    COMMENT(u32),
    LABEL(u32),
}

/// Parse HRM code consisting of instructions (e.g. [Command]) separated by new lines.
/// Returns:
/// - [Ok(Program)] if code was successfully parsed
/// - [Err(ParseError)] else
fn parse_program(code: &str) -> Result<Program, ParseError> {
    let mut program_builder = ProgramBuilder::new();

    for line in code.lines() {
        match parse_line(line)? {
            ParsedLine::Label(label) => program_builder.add_label(label),
            ParsedLine::Command(command) => program_builder.add_command(command),
            ParsedLine::Define(_) => break,
            _ => {}
        }
    }

    Ok(program_builder.build())
}

/// Parse a line of code. Returns:
/// - [Ok(ParsedLine)] if line contains exactly 1 instruction (e.g [Command], comment etc.)
/// - [Err(ParseError)] else
fn parse_line(line: &str) -> Result<ParsedLine, ParseError> {
    let line = line.trim();

    if line.starts_with("--") && line.ends_with("--") {
        return Ok(ParsedLine::CommentedCode);
    }

    if let Some(id) = parse_comment(line) {
        return Ok(ParsedLine::Comment(id));
    }

    if let Some(define_line) = parse_define(line) {
        return Ok(ParsedLine::Define(define_line));
    }

    if let Some(label) = parse_new_label(line) {
        return Ok(ParsedLine::Label(label));
    }

    if let Some(command) = parse_command(line) {
        return Ok(ParsedLine::Command(command));
    }

    Err(IllegalLine(line.to_string()))
}

/// Tries to parse line as a comment. Returns:
/// - [Ok(u32)] if line starts with <code>COMMENT</code> and has an [u32] arg
/// - [None] else
///
/// Expects line to be trimmed.
fn parse_comment(line: &str) -> Option<u32> {
    let regex = Regex::new(r"^COMMENT\s+(\d+)$").unwrap();
    if let Some(captures) = regex.captures(line) {
        let (_, [arg]) = captures.extract();
        return Some(arg.parse().unwrap());
    }

    None
}

/// Tries to parse a define line. Returns:
/// - [Ok(DefineLine)] if define contains the correct type & index
/// - [None] else
///
/// Expects line to be trimmed.
fn parse_define(line: &str) -> Option<DefineLine> {
    let regex = Regex::new(r"^DEFINE\s+(COMMENT|LABEL)\s+(\d+)$").unwrap();
    if let Some(captures) = regex.captures(line) {
        let (_, [define_type, index]) = captures.extract();
        let index = index.parse().unwrap();
        return match define_type {
            "COMMENT" => Some(DefineLine::COMMENT(index)),
            "LABEL" => Some(DefineLine::LABEL(index)),
            &_ => panic!("This cannot occur!"),
        };
    }

    None
}

/// Tries to parse line as a new label. Returns:
/// - [Ok(String)] if line consists of lowercase a-z followed by a colon
/// - [None] else
///
/// Expects line to be trimmed.
fn parse_new_label(line: &str) -> Option<String> {
    let regex = Regex::new(r"^([a-z]+):$").unwrap();
    if let Some(captures) = regex.captures(line) {
        let (_, [label]) = captures.extract();
        return Some(label.to_string());
    }

    None
}

/// Tries to parse a line as a command. Returns:
/// - [Ok(Command)] if line is a valid command with correct args
/// - [None] else
///
/// Expects line to be trimmed.
fn parse_command(line: &str) -> Option<Command> {
    // todo: JUMPa is accepted - assert whitespace between command & arg
    let regex = Regex::new(r"^([A-Z]+)\s*(.*)$").unwrap();
    if let Some(captures) = regex.captures(line) {
        let (_, [command, arg]) = captures.extract();
        return match command {
            "INBOX" => {
                match arg {
                    "" => Some(Command::Inbox),
                    &_ => None,
                }
            }
            "OUTBOX" => {
                match arg {
                    "" => Some(Command::Outbox),
                    &_ => None,
                }
            }
            "COPYFROM" => {
                match parse_value(arg) {
                    Some(value) => Some(Command::CopyFrom(value)),
                    None => None,
                }
            }
            "COPYTO" => {
                match parse_value(arg) {
                    Some(value) => Some(Command::CopyTo(value)),
                    None => None,
                }
            }
            "ADD" => {
                match parse_value(arg) {
                    Some(value) => Some(Command::Add(value)),
                    None => None,
                }
            }
            "SUB" => {
                match parse_value(arg) {
                    Some(value) => Some(Command::Sub(value)),
                    None => None,
                }
            }
            "BUMPUP" => {
                match parse_value(arg) {
                    Some(value) => Some(Command::BumpUp(value)),
                    None => None,
                }
            }
            "BUMPDN" => {
                match parse_value(arg) {
                    Some(value) => Some(Command::BumpDown(value)),
                    None => None,
                }
            }
            "JUMP" => {
                match parse_label(arg) {
                    Some(label) => Some(Command::Jump(label)),
                    None => None,
                }
            }
            "JUMPZ" => {
                match parse_label(arg) {
                    Some(label) => Some(Command::JumpZero(label)),
                    None => None,
                }
            }
            "JUMPN" => {
                match parse_label(arg) {
                    Some(label) => Some(Command::JumpNegative(label)),
                    None => None,
                }
            }
            &_ => None,
        };
    }

    None
}

/// Returns [Ok(Value)] if input matches one of:
/// - <code>\d+</code>
/// - <code>\[\d+\]</code>
///
/// Returns [None] otherwise.
fn parse_value(value: &str) -> Option<Value> {
    let regex = Regex::new(r"^(\[\d+]|\d+)$").unwrap();
    if let Some(captures) = regex.captures(value) {
        let (_, [value]) = captures.extract();
        if value.starts_with("[") {
            let value = (&value[1..(value.len() - 1)]).parse().unwrap();
            return Some(Value::Index(value));
        } else {
            let value = value.parse().unwrap();
            return Some(Value::Value(value));
        }
    }

    None
}

/// Returns [Ok(String)] if input matches <code>\[a-z\]+</code>, else returns [None].
fn parse_label(label: &str) -> Option<String> {
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
    fn parse_comment_succeeds() {
        let line = "COMMENT 123";
        let comment = parse_comment(line).unwrap();
        assert_eq!(123, comment);
    }

    #[test]
    fn parse_comment_fails() {
        for arg in vec!["", "1a", "b", "C", "aBc", "0 1"] {
            let line = format!("COMMENT {}", arg);
            let comment = parse_comment(&line);
            assert!(comment.is_none());
        }
    }

    #[test]
    fn parse_new_label_succeeds() {
        for line in vec!["a:", "abc:"] {
            let label = parse_new_label(line).unwrap();
            assert_eq!(&line[..line.len() - 1], label);
        }
    }

    #[test]
    fn parse_new_label_fails() {
        for line in vec!["INBOX", "A:", "aBc:", "a:b", "a: "] {
            let label = parse_new_label(line);
            assert!(label.is_none());
        }
    }

    #[test]
    fn parse_command_no_arg_succeeds() {
        let command_pairs = [
            ("INBOX", Command::Inbox),
            ("OUTBOX", Command::Outbox),
        ];

        for command_pair in command_pairs {
            let command = parse_command(command_pair.0).unwrap();
            assert_eq!(command_pair.1, command);
        }
    }

    #[test]
    fn parse_command_no_arg_fails() {
        let command_pairs = [
            ("INBOX", Command::Inbox),
            ("OUTBOX", Command::Outbox),
        ];

        for command_pair in command_pairs {
            for arg in ["1", "a", "42b"] {
                let line = format!("{} {}", command_pair.0, arg);
                let command = parse_command(&line);
                assert!(command.is_none());
            }
        }
    }

    #[test]
    fn parse_command_value_arg_succeeds() {
        let value = 123;
        let index = 456;
        let command_pairs: [(&str, fn(Value) -> Command); 6] = [
            ("COPYFROM", Command::CopyFrom),
            ("COPYTO", Command::CopyTo),
            ("ADD", Command::Add),
            ("SUB", Command::Sub),
            ("BUMPUP", Command::BumpUp),
            ("BUMPDN", Command::BumpDown),
        ];

        for command_pair in command_pairs {
            let line = format!("{} {}", command_pair.0, value);
            let command = parse_command(&line).unwrap();
            assert_eq!(command_pair.1(Value::Value(value)), command);

            let line = format!("{} [{}]", command_pair.0, index);
            let command = parse_command(&line).unwrap();
            assert_eq!(command_pair.1(Value::Index(index)), command);
        }
    }

    #[test]
    fn parse_command_value_arg_fails() {
        let command_pairs: [(&str, fn(Value) -> Command); 6] = [
            ("COPYFROM", Command::CopyFrom),
            ("COPYTO", Command::CopyTo),
            ("ADD", Command::Add),
            ("SUB", Command::Sub),
            ("BUMPUP", Command::BumpUp),
            ("BUMPDN", Command::BumpDown),
        ];

        for command_pair in command_pairs {
            for arg in ["", "1a", "abc", "D", "[", "[]", "[1a]", "[A]"] {
                let line = format!("{} {}", command_pair.0, arg);
                let command = parse_command(&line);
                assert!(command.is_none());
            }
        }
    }

    #[test]
    fn parse_command_label_arg_succeeds() {
        let label = "abc";
        let command_pairs: [(&str, fn(String) -> Command); 3] = [
            ("JUMP", Command::Jump),
            ("JUMPZ", Command::JumpZero),
            ("JUMPN", Command::JumpNegative),
        ];

        for command_pair in command_pairs {
            let line = format!("{} {}", command_pair.0, label);
            let command = parse_command(&line).unwrap();
            assert_eq!(command_pair.1(label.to_string()), command);
        }
    }

    #[test]
    fn parse_command_label_arg_fails() {
        let command_pairs: [(&str, fn(String) -> Command); 3] = [
            ("JUMP", Command::Jump),
            ("JUMPZ", Command::JumpZero),
            ("JUMPN", Command::JumpNegative),
        ];

        for command_pair in command_pairs {
            for arg in ["", "aBc", "A", "1"] {
                let line = format!("{} {}", command_pair.0, arg);
                let command = parse_command(&line);
                assert!(command.is_none());
            }
        }
    }

    #[test]
    fn parse_value_empty() {
        let value = parse_value("");
        assert!(value.is_none());
    }

    #[test]
    fn parse_value_value() {
        let value = parse_value("123").unwrap();
        assert_eq!(Value::Value(123), value);
    }

    #[test]
    fn parse_value_index() {
        let value = parse_value("[123]").unwrap();
        assert_eq!(Value::Index(123), value);
    }

    #[test]
    fn parse_label_succeeds() {
        for label in vec!["a", "bc", "def"] {
            let parsed_label = parse_label(label).unwrap();
            assert_eq!(label, parsed_label);
        }
    }

    #[test]
    fn parse_label_fails() {
        for label in vec!["A", "aBc", "1", "a1", "ab:", ""] {
            let label = parse_label(label);
            assert!(label.is_none());
        }
    }
}
