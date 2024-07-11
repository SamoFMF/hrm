use std::fmt::Debug;

use crate::code::{
    game_state::GameState,
    program::{Program, RunError},
};

pub mod add;
pub mod bump_down;
pub mod bump_up;
pub mod copy_from;
pub mod copy_to;
pub mod inbox;
pub mod jump;
pub mod jump_negative;
pub mod jump_zero;
pub mod outbox;
pub mod sub;

pub const ALL_COMMANDS: [&str; 11] = [
    "INBOX", "OUTBOX", "COPYFROM", "COPYTO", "ADD", "SUB", "BUMPUP", "BUMPDN", "JUMP", "JUMPZ",
    "JUMPN",
];

pub type AnyCommand = Box<dyn Command>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CommandValue {
    Value(usize),
    Index(usize),
}

pub trait Command: Debug {
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

#[macro_export]
macro_rules! commands {
    () => {
        vec![
            |cmd, val| {
                <$crate::code::commands::inbox::Inbox as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::outbox::Outbox as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::copy_from::CopyFrom as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::copy_to::CopyTo as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::add::Add as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::sub::Sub as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::bump_up::BumpUp as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::bump_down::BumpDown as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::jump::Jump as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::jump_zero::JumpZero as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
            |cmd, val| {
                <$crate::code::commands::jump_negative::JumpNegative as $crate::code::commands::Command>::create(cmd, val)
                    .map(|cmd| Box::new(cmd) as Box<dyn $crate::code::commands::Command>)
            },
        ]
    };
}
