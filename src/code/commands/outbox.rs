use log::{debug, log_enabled, Level};

use crate::code::program::Program;
use crate::code::{
    commands::command::Command,
    game_state::GameState,
    program::{try_get_acc, RunError},
};

const COMMAND: &str = "OUTBOX";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Outbox;

impl Command for Outbox {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        Outbox::command_static()
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

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<usize, RunError> {
        let value = try_get_acc(game_state.acc)?;

        if log_enabled!(Level::Debug) {
            debug!("Produced value to outbox: {:?}", value);
        }

        if game_state.i_output == game_state.output.len() {
            return Err(RunError::IncorrectOutput {
                expected: None,
                value: Some(value),
            });
        }

        if value != game_state.output[game_state.i_output] {
            return Err(RunError::IncorrectOutput {
                expected: Some(game_state.output[game_state.i_output]),
                value: Some(value),
            });
        }

        game_state.i_output += 1;
        Ok(game_state.i_command + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn command_static_test() {
        assert_eq!(COMMAND, Outbox::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, Outbox.command());
    }

    #[test]
    fn create_succeeds() {
        let command = Outbox::create("OUTBOX", "").unwrap();
        assert_eq!(Outbox, command);
    }

    #[test]
    fn create_fails() {
        let command = Outbox::create("INBOX", "");
        assert_eq!(None, command);

        let command = Outbox::create("OUTBOX", "a");
        assert_eq!(None, command);

        let command = Outbox::create("OUTBOX", "1");
        assert_eq!(None, command);

        let command = Outbox::create("OUTBOX", " ");
        assert_eq!(None, command);
    }

    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![Value::Int(5)],
            memory: vec![],
            acc: Some(Value::Int(5)),
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let i_next = Outbox
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(1, i_next);
        assert_eq!(1, game_state.i_output);
    }

    #[test]
    fn execute_no_outputs() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![],
            acc: Some(Value::Int(5)),
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let result = Outbox
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        let expected = RunError::IncorrectOutput {
            expected: None,
            value: Some(Value::Int(5)),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn execute_bad_output() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![Value::Char('A')],
            memory: vec![],
            acc: Some(Value::Int(5)),
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let result = Outbox
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        let expected = RunError::IncorrectOutput {
            expected: Some(Value::Char('A')),
            value: Some(Value::Int(5)),
        };
        assert_eq!(expected, result);
    }
}
