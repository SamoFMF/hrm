use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

use crate::code::{
    commands::Command,
    game_state::GameState,
    program::{Program, RunError},
};

const COMMAND: &str = "INBOX";

#[derive(Clone, PartialEq)]
pub struct Inbox {
    is_over: RefCell<bool>,
}

impl Debug for Inbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Input")
    }
}

impl Default for Inbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            is_over: RefCell::new(false),
        }
    }
}

impl Command for Inbox {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        Inbox::command_static()
    }

    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if command == COMMAND && args.is_empty() {
            Some(Self::new())
        } else {
            None
        }
    }

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        if game_state.i_input == game_state.input.len() {
            *self.is_over.borrow_mut() = true;
            return Ok(());
        }

        game_state.acc = Some(game_state.input[game_state.i_input]);
        game_state.i_input += 1;
        Ok(())
    }

    fn next(&self, _program: &Program, game_state: &GameState) -> usize {
        if *self.is_over.borrow() {
            usize::MAX
        } else {
            game_state.i_command + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn command_static_test() {
        assert_eq!(COMMAND, Inbox::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, Inbox::new().command());
    }

    #[test]
    fn create_succeeds() {
        let command = Inbox::create("INBOX", "").unwrap();
        assert_eq!(Inbox::new(), command);
    }

    #[test]
    fn create_fails() {
        let command = Inbox::create("OUTBOX", "");
        assert_eq!(None, command);

        let command = Inbox::create("INBOX", "a");
        assert_eq!(None, command);

        let command = Inbox::create("INBOX", "1");
        assert_eq!(None, command);

        let command = Inbox::create("INBOX", " ");
        assert_eq!(None, command);
    }

    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![Value::Int(5)],
            output: &vec![],
            memory: vec![],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        Inbox::new()
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(1, game_state.i_input);
    }

    #[test]
    fn execute_no_inputs() {
        let mut game_state = GameState {
            input: &vec![Value::Int(5)],
            output: &vec![],
            memory: vec![],
            acc: None,
            i_input: 1,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        Inbox::new()
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(1, game_state.i_input);
    }

    #[test]
    fn next_succeeds() {
        let game_state = GameState {
            input: &vec![Value::Int(5)],
            output: &vec![],
            memory: vec![],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        assert_eq!(1, Inbox::new().next(&Default::default(), &game_state));
    }

    #[test]
    fn next_is_over() {
        let game_state = GameState {
            input: &vec![Value::Int(5)],
            output: &vec![],
            memory: vec![],
            acc: None,
            i_input: 1,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        assert_eq!(
            usize::MAX,
            Inbox {
                is_over: RefCell::new(true)
            }
            .next(&Default::default(), &game_state)
        );
    }

    #[test]
    fn requires_index_test() {
        assert!(Inbox::new().requires_index().is_none());
    }

    #[test]
    fn requires_label_test() {
        assert!(Inbox::new().requires_label().is_none());
    }
}
