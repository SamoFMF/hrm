use std::collections::HashMap;

use log::{debug, log_enabled, trace, Level};

use crate::{
    code::{
        commands::{command::AnyCommand, Command, CommandValue},
        game_state::GameState,
    },
    game::{
        problem::{Problem, ProblemIO},
        value::Value,
    },
};

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
    commands_new: Vec<AnyCommand>,
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

    pub fn validate_new(&self, problem: &Problem) -> Result<(), ProgramError> {
        debug!("Validating problem");

        // Validate commands
        for command in &self.commands_new {
            trace!("Validating command: {:?}", command);
            let command_type = command.command();
            if !problem.is_command_available(command_type) {
                return Err(ProgramError::Validation(
                    ValidationError::CommandNotAvailable(command_type.to_string()),
                ));
            }

            if let Some(idx) = command.requires_index() {
                if idx >= problem.get_memory().len() {
                    return Err(ProgramError::Validation(ValidationError::CommandIndex(idx)));
                }
            }

            if let Some(label) = command.requires_label() {
                if !self.labels.contains_key(label) {
                    return Err(ProgramError::Validation(ValidationError::MissingLabel(
                        label.to_string(),
                    )));
                }
            }
        }

        // Validate labels
        for (label, &idx) in &self.labels {
            trace!("Validating label: {} => {}", label, idx);
            if idx > self.commands_new.len() {
                return Err(ProgramError::Validation(ValidationError::LabelIndex(idx)));
            }
        }

        debug!("Successfully validated program");

        Ok(())
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
            debug!("Successfully finished problem for all IOs");
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
        let mut game_state = GameState::new(&problem_io.input, &problem_io.output, memory);

        while game_state.i_command < self.commands_new.len() {
            game_state.speed += 1;
            let command = &self.commands_new[game_state.i_command];
            trace!("Running command {}: {:?}", game_state.i_command, command);

            command.execute(&self, &mut game_state)?;
            game_state.i_command = command.next(&self, &game_state);
        }

        if game_state.i_output == game_state.output.len() {
            let speed_delta = if game_state.i_command == self.commands_new.len() {
                debug!("No more commands to execute");
                0 // No more commands to be executed
            } else {
                debug!("No more inputs to consume");
                1 // Ended on Inbox - remove from count
            };

            Ok(game_state.speed - speed_delta)
        } else {
            Err(RunError::IncorrectOutput {
                expected: Some(game_state.output[game_state.i_output]),
                value: None,
            })
        }
    }
}

pub fn try_get_acc(acc: Option<Value>) -> Result<Value, RunError> {
    match acc {
        Some(acc) => Ok(acc),
        None => Err(RunError::EmptyAccNew),
    }
}

pub fn try_get_from_memory(memory: Option<Value>) -> Result<Value, RunError> {
    match memory {
        Some(value) => Ok(value),
        None => Err(RunError::EmptyMemoryNew),
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

pub struct ProgramBuilder {
    commands: Vec<Command>,
    commands_new: Vec<AnyCommand>,
    labels: HashMap<String, usize>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            commands_new: vec![],
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

    pub fn add_command_ref_new(&mut self, command: AnyCommand) {
        self.commands_new.push(command);
    }

    pub fn add_command_new(mut self, command: AnyCommand) -> Self {
        self.add_command_ref_new(command);
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
            commands_new: self.commands_new,
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
                    commands_new: vec![], // todo
                    labels: Default::default(),
                },
                ProgramError::Validation(ValidationError::CommandIndex(dim + 1)),
            ),
            (
                Program {
                    commands: vec![Command::Jump(String::from("a"))],
                    commands_new: vec![], // todo
                    labels: Default::default(),
                },
                ProgramError::Validation(ValidationError::MissingLabel(String::from("a"))),
            ),
            (
                Program {
                    commands: vec![],
                    commands_new: vec![], // todo
                    labels: HashMap::from([(String::from("a"), dim + 1)]),
                },
                ProgramError::Validation(ValidationError::LabelIndex(dim + 1)),
            ),
            (
                Program {
                    commands: vec![Command::Sub(CommandValue::Value(0))],
                    commands_new: vec![], // todo
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
