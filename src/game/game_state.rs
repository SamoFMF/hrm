use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct GameState { // todo: add available commands
    ios: Vec<GameIO>,
    memory: Vec<Option<Value>>,
}

impl GameState {
    pub fn get_ios(&self) -> &Vec<GameIO> {
        &self.ios
    }

    pub fn get_memory(&self) -> &Vec<Option<Value>> {
        &self.memory
    }
}

pub struct GameStateBuilder {
    ios: Vec<GameIO>,
    memory: HashMap<usize, Value>,
    memory_dim: Option<usize>,
}

impl GameStateBuilder {
    pub fn new() -> Self {
        Self {
            ios: vec![],
            memory: Default::default(),
            memory_dim: None,
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

    pub fn build(self) -> GameState {
        if self.ios.is_empty() {
            panic!("No IO values set!");
        }

        let mut memory = match self.memory_dim {
            Some(memory_dim) => vec![None; memory_dim],
            None => panic!("Memory dimension not set!"),
        };

        for (i, value) in self.memory {
            if i < 0 || i >= memory.len() {
                panic!("Contains memory values outside 0..memory_dim!");
            }

            memory[i] = Some(value);
        }

        GameState {
            ios: self.ios,
            memory,
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
