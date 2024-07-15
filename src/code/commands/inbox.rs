use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory},
        game_state::GameState,
        program::{Program, RunError},
    },
    create_with_args,
};

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

    fn create(args: &str) -> Option<Self> {
        if args.is_empty() {
            Some(Inbox::new())
        } else {
            None
        }
    }
}

impl Command for Inbox {
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

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(InboxFactory)
    }
}

pub struct InboxFactory;

impl CommandFactory for InboxFactory {
    fn command(&self) -> &'static str {
        "INBOX"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(Inbox, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:inbox
    #[test]
    fn create_succeeds() {
        let command = Inbox::create("").unwrap();
        assert_eq!(Inbox::new(), command);
    }

    #[test]
    fn create_fails() {
        let command = Inbox::create("a");
        assert!(command.is_none());

        let command = Inbox::create("1");
        assert!(command.is_none());

        let command = Inbox::create(" ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("INBOX", InboxFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = InboxFactory.create("");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = InboxFactory.create("a");
        assert!(command.is_none());

        let command = InboxFactory.create("1");
        assert!(command.is_none());

        let command = InboxFactory.create(" ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
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

    #[test]
    fn factory_test() {
        assert_eq!("INBOX", Inbox::new().factory().command());
    }
    // endregion
}
