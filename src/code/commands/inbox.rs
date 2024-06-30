use crate::code::commands::command::Command;
use crate::code::game_state::GameState;
use crate::code::program::{Program, RunError};

const COMMAND: &str = "INBOX";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inbox;

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
            Some(Self)
        } else {
            None
        }
    }

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        if game_state.i_input == game_state.input.len() {
            return Ok(());
        }

        game_state.acc = Some(game_state.input[game_state.i_input]);
        game_state.i_input += 1;
        Ok(())
    }

    fn next(&self, _program: &Program, game_state: &GameState) -> usize {
        if game_state.i_input == game_state.input.len() {
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
        assert_eq!(COMMAND, Inbox.command());
    }

    #[test]
    fn create_succeeds() {
        let command = Inbox::create("INBOX", "").unwrap();
        assert_eq!(Inbox, command);
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

        Inbox.execute(&Default::default(), &mut game_state).unwrap();
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

        Inbox.execute(&Default::default(), &mut game_state).unwrap();
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

        assert_eq!(1, Inbox.next(&Default::default(), &game_state));
    }

    #[test]
    fn next_no_inputs() {
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

        assert_eq!(usize::MAX, Inbox.next(&Default::default(), &game_state));
    }
}
