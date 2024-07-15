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

    /// Factory
    ///
    /// Returns factory for given command.
    fn factory(&self) -> Box<dyn CommandFactory>;
}

pub trait CommandFactory {
    /// Command
    ///
    /// Returns command keyword.
    fn command(&self) -> &'static str;

    /// Create Command
    ///
    /// Returns [Some(AnyCommand)] if successful, else [None].
    fn create(&self, args: &str) -> Option<AnyCommand>;
}

#[macro_export]
macro_rules! create_with_args {
    ($t:ty, $args:expr) => {
        <$t>::create($args).map(|value| Box::new(value) as $crate::code::commands::AnyCommand)
    };
}

#[macro_export]
macro_rules! commands {
    () => {
        vec![
            Box::new($crate::code::commands::add::AddFactory),
            Box::new($crate::code::commands::bump_down::BumpDownFactory),
            Box::new($crate::code::commands::bump_up::BumpUpFactory),
            Box::new($crate::code::commands::copy_from::CopyFromFactory),
            Box::new($crate::code::commands::copy_to::CopyToFactory),
            Box::new($crate::code::commands::inbox::InboxFactory),
            Box::new($crate::code::commands::jump::JumpFactory),
            Box::new($crate::code::commands::jump_negative::JumpNegativeFactory),
            Box::new($crate::code::commands::jump_zero::JumpZeroFactory),
            Box::new($crate::code::commands::outbox::OutboxFactory),
            Box::new($crate::code::commands::sub::SubFactory),
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_macro_test() {
        let expected = [
            "INBOX", "OUTBOX", "ADD", "SUB", "BUMPUP", "BUMPDN", "COPYTO", "COPYFROM", "JUMP",
            "JUMPN", "JUMPZ",
        ];
        let cmds: Vec<Box<dyn CommandFactory>> = commands!();

        assert_eq!(expected.len(), cmds.len());
        for cmd in cmds {
            assert!(expected.contains(&cmd.command()));
        }
    }
}
