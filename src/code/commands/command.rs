use std::fmt::Debug;

use crate::code::{
    game_state::GameState,
    program::{Program, RunError},
};

pub type AnyCommand = Box<dyn CommandNew>;

pub trait CommandNew: Debug {
    /// Command
    ///
    /// Get command keyword
    fn command_static() -> &'static str
    where
        Self: Sized;

    /// Command On Object
    ///
    /// Get command keyword on object
    fn command(&self) -> &'static str;

    /// Try Parse Instruction
    ///
    /// Try to parse a command with args into [Self],
    /// returns [Some(Self)] if it succeeds, else [None].
    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized;

    /// Execute
    ///
    /// Execute the command & return the index of the next command.
    fn execute(&self, program: &Program, game_state: &mut GameState) -> Result<(), RunError>;

    /// Next
    ///
    /// Get next command index
    fn next(&self, _program: &Program, game_state: &GameState) -> usize {
        game_state.i_command + 1
    }

    /// Requires Index
    ///
    /// Returns [Some(usize)] if an index must exist for the command to work, else [None].
    fn requires_index(&self) -> Option<usize> {
        None
    }

    /// Requires Label
    ///
    /// Returns [Some(&str)] if a label must exist for the command to work, else [None].
    fn requires_label(&self) -> Option<&str> {
        None
    }
}
