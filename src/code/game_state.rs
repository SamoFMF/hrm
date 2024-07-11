use crate::code::program::Memory;
use crate::game::value::Value;

pub struct GameState<'a> {
    pub input: &'a Vec<Value>,
    pub output: &'a Vec<Value>,
    pub memory: Memory,
    pub acc: Option<Value>,
    pub i_input: usize,
    pub i_output: usize,
    pub i_command: usize,
    pub speed: u32,
}

impl<'a> GameState<'a> {
    pub fn new(input: &'a Vec<Value>, output: &'a Vec<Value>, memory: Memory) -> Self {
        Self {
            input,
            output,
            memory,
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        }
    }
}
