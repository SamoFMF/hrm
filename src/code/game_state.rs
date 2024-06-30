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
