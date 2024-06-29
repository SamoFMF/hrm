use std::collections::HashMap;
use std::ops::Add;

use crate::code::commands::{Command, CommandValue};
use crate::game::problem::{Problem, ProblemIO};
use crate::game::value::Value;

type Memory = Vec<Option<Value>>;

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    Validation(ValidationError),
    Run(RunError),
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    CommandNotAvailable(String),
    CommandIndex(usize),
    MissingLabel(String),
    LabelIndex(usize),
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    EmptyAcc(Command),
    EmptyMemory(Command),
    IncorrectOutput { expected: Option<Value>, value: Option<Value> },
    CharIndex(Value),
    IndexOutOfRange(Value),
    Add(Command),
    Sub(Command),
}

#[derive(Debug)]
pub struct Program {
    // todo: add comments & labels - verify them
    commands: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl Program {
    pub fn validate(&self, problem: &Problem) -> Result<(), ProgramError> {
        // Verify commands
        for command in &self.commands {
            if *command == Command::End {
                continue;
            }
            let command_type = command.get_type();
            if !problem.is_command_available(&command_type) {
                return Err(ProgramError::Validation(ValidationError::CommandNotAvailable(command_type)));
            }

            match command {
                Command::CopyFrom(value) | Command::CopyTo(value)
                | Command::Add(value) | Command::Sub(value)
                | Command::BumpUp(value) | Command::BumpDown(value) => {
                    let idx = match value {
                        CommandValue::Value(value) => { *value }
                        CommandValue::Index(index) => { *index }
                    };

                    if idx >= problem.get_memory().len() {
                        return Err(ProgramError::Validation(ValidationError::CommandIndex(idx)));
                    }
                }
                Command::Jump(label) | Command::JumpZero(label) | Command::JumpNegative(label) => {
                    if !self.labels.contains_key(label) {
                        return Err(ProgramError::Validation(ValidationError::MissingLabel(label.clone())));
                    }
                }
                &_ => {}
            }
        }

        // Verify labels
        for (_, idx) in &self.labels {
            if *idx > self.commands.len() {
                return Err(ProgramError::Validation(ValidationError::LabelIndex(*idx)));
            }
        }

        Ok(())
    }

    /// todo: add section for panics:
    /// - can panic if [Program::validate] is not run first, e.g. labels are not guaranteed to exist
    /// and unwrap will panic
    pub fn run(&self, problem: &Problem) -> Result<(), RunError> {
        for problem_io in problem.get_ios() {
            self.run_io(problem_io, problem.get_memory().clone())?;
        }

        Ok(())
    }

    fn run_io(&self, problem_io: &ProblemIO, mut memory: Memory) -> Result<(), RunError> {
        let ProblemIO { input, output } = problem_io;
        let mut acc = None;
        let mut i_input = 0;
        let mut i_output = 0;
        let mut i_command = 0;

        loop { // todo: replace with while i_command < len & remove END command
            let command = &self.commands[i_command];
            println!("Running command {i_command}: {:?}", command);
            match command {
                Command::Inbox => {
                    if i_input == input.len() {
                        break;
                    }

                    acc = Some(input[i_input]);
                    i_input += 1;
                }
                Command::Outbox => {
                    let value = get_acc(acc, command)?;

                    if i_output == output.len() {
                        return Err(RunError::IncorrectOutput {
                            expected: None,
                            value: acc,
                        });
                    }

                    if value != output[i_output] {
                        return Err(RunError::IncorrectOutput {
                            expected: Some(output[i_output]),
                            value: Some(value),
                        });
                    }

                    i_output += 1;
                }
                Command::CopyFrom(command_value) => {
                    let index = get_index(command_value, &memory, command)?;
                    acc = Some(get_from_memory(memory[index], command)?);
                }
                Command::CopyTo(command_value) => {
                    let value = get_acc(acc, command)?;
                    let index = get_index(command_value, &memory, command)?;
                    memory[index] = Some(value);
                }
                Command::Add(command_value) => {
                    let value = get_acc(acc, command)?;
                    let index = get_index(command_value, &memory, command)?;
                    let to_add = get_from_memory(memory[index], command)?;
                    let sum = value.add(to_add).ok_or(RunError::Add(command.clone()))?;
                    acc = Some(sum);
                }
                Command::Sub(command_value) => {
                    let value = get_acc(acc, command)?;
                    let index = get_index(command_value, &memory, command)?;
                    let to_sub = get_from_memory(memory[index], command)?;
                    let diff = value.sub(to_sub).ok_or(RunError::Sub(command.clone()))?;
                    acc = Some(diff);
                }
                Command::BumpUp(command_value) => {
                    let index = get_index(command_value, &memory, command)?;
                    let to_bump = get_from_memory(memory[index], command)?;
                    let bumped = to_bump.add(Value::INT(1)).ok_or(RunError::Add(command.clone()))?;
                    acc = Some(bumped);
                }
                Command::BumpDown(command_value) => {
                    let index = get_index(command_value, &memory, command)?;
                    let to_bump = get_from_memory(memory[index], command)?;
                    let bumped = to_bump.sub(Value::INT(1)).ok_or(RunError::Sub(command.clone()))?;
                    acc = Some(bumped);
                }
                Command::Jump(label) => {
                    let index = *self.labels.get(label).unwrap(); // safe if program is validated
                    i_command = index;
                    continue;
                }
                Command::JumpZero(label) => {
                    let value = get_acc(acc, command)?;
                    if value == 0 {
                        let index = *self.labels.get(label).unwrap(); // safe if program is validated
                        i_command = index;
                        continue;
                    }
                }
                Command::JumpNegative(label) => {
                    let value = get_acc(acc, command)?;
                    if value < 0 {
                        let index = *self.labels.get(label).unwrap(); // safe if program is validated
                        i_command = index;
                        continue;
                    }
                }
                Command::End => break,
            }

            i_command += 1;
        }

        if i_output == output.len() {
            return Ok(());
        }

        Err(RunError::IncorrectOutput {
            expected: Some(output[i_output]),
            value: None,
        })
    }
}

fn get_acc(acc: Option<Value>, command: &Command) -> Result<Value, RunError> {
    match acc {
        Some(acc) => Ok(acc),
        None => Err(RunError::EmptyAcc(command.clone())),
    }
}

fn get_from_memory(memory: Option<Value>, command: &Command) -> Result<Value, RunError> {
    match memory {
        Some(value) => Ok(value),
        None => Err(RunError::EmptyMemory(command.clone())),
    }
}

fn get_index(command_value: &CommandValue, memory: &Memory, command: &Command) -> Result<usize, RunError> {
    match command_value {
        CommandValue::Value(value) => Ok(*value),
        CommandValue::Index(index) => {
            let index_value = get_from_memory(memory[*index], command)?;
            match index_value {
                Value::INT(idx) => {
                    if idx < 0 || idx as usize >= memory.len() {
                        Err(RunError::IndexOutOfRange(index_value))
                    } else {
                        Ok(idx as usize)
                    }
                }
                Value::CHAR(_) => Err(RunError::CharIndex(index_value))
            }
        }
    }
}

pub struct ProgramBuilder {
    commands: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            labels: HashMap::new(),
        }
    }

    pub fn add_command_ref(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn add_command(mut self, command: Command) -> Self {
        self.add_command_ref(command);
        self
    }

    pub fn add_label_ref(&mut self, label: String) {
        self.labels.insert(label, self.commands.len());
    }

    pub fn add_label(mut self, label: String) -> Self {
        self.add_label_ref(label);
        self
    }

    pub fn build(mut self) -> Program {
        self.commands.push(Command::End);
        Program {
            commands: self.commands,
            labels: self.labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::problem::{ProblemBuilder, ProblemIO};

    use super::*;

    #[test]
    fn validate_succeeds() {
        let problem = ProblemBuilder::new()
            .memory_dim(5)
            .add_io(ProblemIO { input: vec![], output: vec![] })
            .enable_all_commands()
            .build();

        let program = ProgramBuilder::new()
            .add_label(String::from("a"))
            .add_command(Command::CopyFrom(CommandValue::Value(0)))
            .add_label(String::from("b"))
            .add_command(Command::CopyTo(CommandValue::Index(4)))
            .add_label(String::from("c"))
            .add_command(Command::Jump(String::from("a")))
            .build();

        program.validate(&problem).unwrap();
    }

    #[test]
    fn validate_fails() {
        let dim = 5;
        let problem = ProblemBuilder::new()
            .memory_dim(dim)
            .add_io(ProblemIO { input: vec![], output: vec![] })
            .enable_all_commands()
            .disable_command("SUB")
            .build();

        let validate_results = [
            (Program {
                commands: vec![Command::Add(CommandValue::Index(dim + 1))],
                labels: Default::default(),
            }, ProgramError::Validation(ValidationError::CommandIndex(dim + 1))),
            (Program {
                commands: vec![Command::Jump(String::from("a"))],
                labels: Default::default(),
            }, ProgramError::Validation(ValidationError::MissingLabel(String::from("a")))),
            (Program {
                commands: vec![],
                labels: HashMap::from([(String::from("a"), dim + 1)]),
            }, ProgramError::Validation(ValidationError::LabelIndex(dim + 1))),
            (Program {
                commands: vec![Command::Sub(CommandValue::Value(0))],
                labels: HashMap::from([(String::from("a"), dim + 1)]),
            }, ProgramError::Validation(ValidationError::CommandNotAvailable(String::from("SUB")))),
        ];

        for validate_result in validate_results {
            let err = match validate_result.0.validate(&problem) {
                Ok(_) => panic!("Expected to fail!"),
                Err(err) => err,
            };
            assert_eq!(validate_result.1, err);
        }
    }
}
