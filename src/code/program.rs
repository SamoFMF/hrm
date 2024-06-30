use std::collections::HashMap;

use log::{debug, log_enabled, trace, Level};

use crate::code::commands::{command, inbox, outbox, Command, CommandValue};
use crate::code::game_state::GameState;
use crate::game::problem::{Problem, ProblemIO};
use crate::game::value::Value;

pub type Memory = Vec<Option<Value>>;

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
    EmptyAccNew,
    EmptyMemory(Command),
    EmptyMemoryNew,
    IncorrectOutput {
        expected: Option<Value>,
        value: Option<Value>,
    },
    CharIndex(Value),
    IndexOutOfRange(Value),
    Add(Command),
    AddNew,
    Sub(Command),
    SubNew,
}

#[derive(Debug, PartialEq)]
pub struct Score {
    pub size: usize,
    pub speed_min: u32,
    pub speed_max: u32,
    pub speed_avg: f64,
}

#[derive(Debug, Default)]
pub struct Program {
    // todo: add comments & defines - verify them
    commands: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl Program {
    /// Get Label
    ///
    /// Get label's index.
    ///
    /// # Panics
    ///
    /// Panics if the label does not exist. Will NEVER panic if the program
    /// is validated with [Program::validate].
    pub fn get_label(&self, label: &str) -> usize {
        *self.labels.get(label).unwrap() // safe if program is validated
    }

    pub fn validate(&self, problem: &Problem) -> Result<(), ProgramError> {
        if log_enabled!(Level::Debug) {
            debug!("Validating problem");
        }

        // Verify commands
        for command in &self.commands {
            if log_enabled!(Level::Trace) {
                trace!("Validating command: {:?}", command);
            }
            if *command == Command::End {
                continue;
            }
            let command_type = command.get_type();
            if !problem.is_command_available(&command_type) {
                return Err(ProgramError::Validation(
                    ValidationError::CommandNotAvailable(command_type),
                ));
            }

            match command {
                Command::CopyFrom(value)
                | Command::CopyTo(value)
                | Command::Add(value)
                | Command::Sub(value)
                | Command::BumpUp(value)
                | Command::BumpDown(value) => {
                    let idx = match value {
                        CommandValue::Value(value) => *value,
                        CommandValue::Index(index) => *index,
                    };

                    if idx >= problem.get_memory().len() {
                        return Err(ProgramError::Validation(ValidationError::CommandIndex(idx)));
                    }
                }
                Command::Jump(label) | Command::JumpZero(label) | Command::JumpNegative(label) => {
                    if !self.labels.contains_key(label) {
                        return Err(ProgramError::Validation(ValidationError::MissingLabel(
                            label.clone(),
                        )));
                    }
                }
                &_ => {}
            }
        }

        // Verify labels
        for (_, idx) in &self.labels {
            if log_enabled!(Level::Trace) {
                trace!("Verifying label: {:?}", *idx);
            }
            if *idx > self.commands.len() {
                return Err(ProgramError::Validation(ValidationError::LabelIndex(*idx)));
            }
        }

        if log_enabled!(Level::Debug) {
            debug!("Successfully validated program");
        }
        Ok(())
    }

    /// Run code
    ///
    /// Run [Program] for given [Problem].
    ///
    /// # Panics
    ///
    /// Labels are not guaranteed to exist without running [Program::validate], which can cause
    /// program to panic when unwrapping.
    pub fn run(&self, problem: &Problem) -> Result<Score, RunError> {
        if log_enabled!(Level::Debug) {
            debug!("Running program");
        }

        let (mut speed_min, mut speed_max, mut speed_avg) = (u32::MAX, 0, 0);
        for problem_io in problem.get_ios() {
            // let speed = self.run_io(problem_io, problem.get_memory().clone())?;
            let speed = self.run_io_new(problem_io, problem.get_memory().clone())?;

            if log_enabled!(Level::Debug) {
                debug!("Program ended, speed = {speed}");
            }

            if speed > speed_max {
                speed_max = speed;
            }

            if speed < speed_min {
                speed_min = speed;
            }

            speed_avg += speed;
        }

        if log_enabled!(Level::Debug) {
            debug!("Successfully finished problem for all IO");
        }

        Ok(Score {
            size: self.commands.len() - 1, // sub END
            speed_min,
            speed_max,
            speed_avg: (speed_avg as f64) / (problem.get_ios().len() as f64),
        })
    }

    fn run_io_new(&self, problem_io: &ProblemIO, memory: Memory) -> Result<u32, RunError> {
        if log_enabled!(Level::Debug) {
            debug!("Running program for new IO");
        }
        let mut game_state = GameState {
            input: &problem_io.input,
            output: &problem_io.output,
            memory,
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let commands: &Vec<Box<dyn command::Command>> = &vec![
            Box::new(inbox::Inbox),
            Box::new(outbox::Outbox),
            Box::new(inbox::Inbox),
            Box::new(outbox::Outbox),
            Box::new(inbox::Inbox),
            Box::new(outbox::Outbox),
        ];

        while game_state.i_command < commands.len() {
            game_state.speed += 1;
            let command = &commands[game_state.i_command];

            println!(
                "Executing: {}, i_command = {}",
                command.command(),
                game_state.i_command
            );

            command.execute(&self, &mut game_state)?;
            game_state.i_command = command.next(&self, &game_state);
        }

        println!("i_command = {}", game_state.i_command);
        if game_state.i_output == game_state.output.len() {
            let speed_delta = if game_state.i_command == commands.len() {
                0 // No more commands to be executed
            } else {
                1 // Ended on Inbox - remove from count
            };

            return Ok(game_state.speed - speed_delta);
        }

        Err(RunError::IncorrectOutput {
            expected: Some(game_state.output[game_state.i_output]),
            value: None,
        })
    }

    fn run_io(&self, problem_io: &ProblemIO, memory: Memory) -> Result<u32, RunError> {
        if log_enabled!(Level::Debug) {
            debug!("Running program for new IO");
        }
        let mut game_state = GameState {
            input: &problem_io.input,
            output: &problem_io.output,
            memory,
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        loop {
            game_state.speed += 1;
            let command = &self.commands[game_state.i_command];
            if log_enabled!(Level::Trace) {
                trace!("Running command {}: {:?}", game_state.i_command, command);
            }

            match command {
                Command::Inbox => {
                    if game_state.i_input == game_state.input.len() {
                        break;
                    }

                    game_state.acc = Some(game_state.input[game_state.i_input]);
                    game_state.i_input += 1;
                }
                Command::Outbox => {
                    let value = get_acc(game_state.acc, command)?;

                    if log_enabled!(Level::Debug) {
                        debug!("Produced value to outbox: {:?}", value);
                    }

                    if game_state.i_output == game_state.output.len() {
                        return Err(RunError::IncorrectOutput {
                            expected: None,
                            value: game_state.acc,
                        });
                    }

                    if value != game_state.output[game_state.i_output] {
                        return Err(RunError::IncorrectOutput {
                            expected: Some(game_state.output[game_state.i_output]),
                            value: Some(value),
                        });
                    }

                    game_state.i_output += 1;
                }
                Command::CopyFrom(command_value) => {
                    let index = get_index(command_value, &game_state.memory, command)?;
                    game_state.acc = Some(get_from_memory(game_state.memory[index], command)?);
                }
                Command::CopyTo(command_value) => {
                    let value = get_acc(game_state.acc, command)?;
                    let index = get_index(command_value, &game_state.memory, command)?;
                    game_state.memory[index] = Some(value);
                }
                Command::Add(command_value) => {
                    let value = get_acc(game_state.acc, command)?;
                    let index = get_index(command_value, &game_state.memory, command)?;
                    let to_add = get_from_memory(game_state.memory[index], command)?;
                    let sum = value.add(to_add).ok_or(RunError::Add(command.clone()))?;
                    game_state.acc = Some(sum);
                }
                Command::Sub(command_value) => {
                    let value = get_acc(game_state.acc, command)?;
                    let index = get_index(command_value, &game_state.memory, command)?;
                    let to_sub = get_from_memory(game_state.memory[index], command)?;
                    let diff = value.sub(to_sub).ok_or(RunError::Sub(command.clone()))?;
                    game_state.acc = Some(diff);
                }
                Command::BumpUp(command_value) => {
                    let index = get_index(command_value, &game_state.memory, command)?;
                    let to_bump = get_from_memory(game_state.memory[index], command)?;
                    let bumped = to_bump
                        .add(Value::Int(1))
                        .ok_or(RunError::Add(command.clone()))?;
                    game_state.memory[index] = Some(bumped);
                    game_state.acc = Some(bumped);
                }
                Command::BumpDown(command_value) => {
                    let index = get_index(command_value, &game_state.memory, command)?;
                    let to_bump = get_from_memory(game_state.memory[index], command)?;
                    let bumped = to_bump
                        .sub(Value::Int(1))
                        .ok_or(RunError::Sub(command.clone()))?;
                    game_state.memory[index] = Some(bumped);
                    game_state.acc = Some(bumped);
                }
                Command::Jump(label) => {
                    let index = self.get_label(label);
                    game_state.i_command = index;
                    continue;
                }
                Command::JumpZero(label) => {
                    let value = get_acc(game_state.acc, command)?;
                    if value == 0 {
                        let index = self.get_label(label);
                        game_state.i_command = index;
                        continue;
                    }
                }
                Command::JumpNegative(label) => {
                    let value = get_acc(game_state.acc, command)?;
                    if value < 0 {
                        let index = *self.labels.get(label).unwrap(); // safe if program is validated
                        game_state.i_command = index;
                        continue;
                    }
                }
                Command::End => break,
            }

            game_state.i_command += 1;
        }

        if game_state.i_output == game_state.output.len() {
            return Ok(game_state.speed - 1); // Inbox / End must not be counted
        }

        Err(RunError::IncorrectOutput {
            expected: Some(game_state.output[game_state.i_output]),
            value: None,
        })
    }
}

pub fn try_get_acc(acc: Option<Value>) -> Result<Value, RunError> {
    match acc {
        Some(acc) => Ok(acc),
        None => Err(RunError::EmptyAccNew),
    }
}

fn get_acc(acc: Option<Value>, command: &Command) -> Result<Value, RunError> {
    match acc {
        Some(acc) => Ok(acc),
        None => Err(RunError::EmptyAcc(command.clone())),
    }
}

pub fn try_get_from_memory(memory: Option<Value>) -> Result<Value, RunError> {
    match memory {
        Some(value) => Ok(value),
        None => Err(RunError::EmptyMemoryNew),
    }
}

fn get_from_memory(memory: Option<Value>, command: &Command) -> Result<Value, RunError> {
    match memory {
        Some(value) => Ok(value),
        None => Err(RunError::EmptyMemory(command.clone())),
    }
}

pub fn try_get_index(command_value: &CommandValue, memory: &Memory) -> Result<usize, RunError> {
    match command_value {
        CommandValue::Value(value) => Ok(*value),
        CommandValue::Index(index) => {
            let index_value = try_get_from_memory(memory[*index])?;
            match index_value {
                Value::Int(idx) => {
                    if idx < 0 || idx as usize >= memory.len() {
                        Err(RunError::IndexOutOfRange(index_value))
                    } else {
                        Ok(idx as usize)
                    }
                }
                Value::Char(_) => Err(RunError::CharIndex(index_value)),
            }
        }
    }
}

fn get_index(
    command_value: &CommandValue,
    memory: &Memory,
    command: &Command,
) -> Result<usize, RunError> {
    match command_value {
        CommandValue::Value(value) => Ok(*value),
        CommandValue::Index(index) => {
            let index_value = get_from_memory(memory[*index], command)?;
            match index_value {
                Value::Int(idx) => {
                    if idx < 0 || idx as usize >= memory.len() {
                        Err(RunError::IndexOutOfRange(index_value))
                    } else {
                        Ok(idx as usize)
                    }
                }
                Value::Char(_) => Err(RunError::CharIndex(index_value)),
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
            .add_io(ProblemIO {
                input: vec![],
                output: vec![],
            })
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
            .add_io(ProblemIO {
                input: vec![],
                output: vec![],
            })
            .enable_all_commands()
            .disable_command("SUB")
            .build();

        let validate_results = [
            (
                Program {
                    commands: vec![Command::Add(CommandValue::Index(dim + 1))],
                    labels: Default::default(),
                },
                ProgramError::Validation(ValidationError::CommandIndex(dim + 1)),
            ),
            (
                Program {
                    commands: vec![Command::Jump(String::from("a"))],
                    labels: Default::default(),
                },
                ProgramError::Validation(ValidationError::MissingLabel(String::from("a"))),
            ),
            (
                Program {
                    commands: vec![],
                    labels: HashMap::from([(String::from("a"), dim + 1)]),
                },
                ProgramError::Validation(ValidationError::LabelIndex(dim + 1)),
            ),
            (
                Program {
                    commands: vec![Command::Sub(CommandValue::Value(0))],
                    labels: HashMap::from([(String::from("a"), dim + 1)]),
                },
                ProgramError::Validation(ValidationError::CommandNotAvailable(String::from("SUB"))),
            ),
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
