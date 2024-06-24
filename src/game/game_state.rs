use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

use crate::code::commands::ALL_COMMANDS;

#[derive(Debug)]
pub struct GameState { // todo: add available commands
    ios: Vec<GameIO>,
    memory: Vec<Option<Value>>,
    available_commands: HashSet<String>,
}

impl GameState {
    pub fn get_ios(&self) -> &Vec<GameIO> {
        &self.ios
    }

    pub fn get_memory(&self) -> &Vec<Option<Value>> {
        &self.memory
    }

    pub fn is_command_available(&self, command: &str) -> bool {
        self.available_commands.contains(command)
    }
}

pub struct GameStateBuilder {
    ios: Vec<GameIO>,
    memory: HashMap<usize, Value>,
    memory_dim: Option<usize>,
    available_commands: HashSet<String>,
}

impl GameStateBuilder {
    pub fn new() -> Self {
        Self {
            ios: vec![],
            memory: Default::default(),
            memory_dim: None,
            available_commands: Default::default(),
        }
    }

    pub fn add_io(mut self, game_io: GameIO) -> Self {
        self.ios.push(game_io);
        self
    }

    pub fn memory_dim(mut self, dim: usize) -> Self {
        self.memory_dim = Some(dim);
        self
    }

    pub fn add_memory_slot(mut self, slot: usize, value: Value) -> Self {
        self.memory.insert(slot, value);
        self
    }

    pub fn enable_all_commands(mut self) -> Self {
        // todo: tests
        self.available_commands = HashSet::from_iter(
            ALL_COMMANDS.iter().map(|command| command.to_string())
        );
        self
    }

    pub fn enable_command(mut self, command: &str) -> Self {
        // todo: tests
        if ALL_COMMANDS.contains(&command) {
            self.available_commands.insert(command.to_string());
        }
        self
    }

    pub fn disable_command(mut self, command: &str) -> Self {
        // todo: tests
        self.available_commands.remove(command);
        self
    }

    pub fn build(self) -> GameState {
        if self.ios.is_empty() {
            panic!("No IO values set!");
        }

        let mut memory = match self.memory_dim {
            Some(memory_dim) => vec![None; memory_dim],
            None => panic!("Memory dimension not set!"),
        };

        for (i, value) in self.memory {
            if i >= memory.len() {
                panic!("Contains memory values outside 0..memory_dim!");
            }

            memory[i] = Some(value);
        }

        GameState {
            ios: self.ios,
            memory,
            available_commands: self.available_commands,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameIO {
    pub input: Vec<Value>,
    pub output: Vec<Value>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    INT(i32),
    CHAR(u8),
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        // todo - might need to implement custom add method that returns Result<Value,?>
        match (self, rhs) {
            (Value::INT(lhs), Value::INT(rhs)) => Value::INT(lhs + rhs),
            (Value::CHAR(lhs), Value::CHAR(rhs)) => Value::INT(lhs as i32 + rhs as i32),
            _ => panic!("Cannot add / sub INT & CHAR"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        // todo - might need to implement custom sub method that returns Result<Value,?>
        match (self, rhs) {
            (Value::INT(lhs), Value::INT(rhs)) => Value::INT(lhs - rhs),
            (Value::CHAR(lhs), Value::CHAR(rhs)) => Value::INT(lhs as i32 - rhs as i32),
            _ => panic!("Cannot add / sub INT & CHAR"),
        }
    }
}
