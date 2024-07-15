use log::{debug, log_enabled, Level};

use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory},
        game_state::GameState,
        program::Program,
        program::{get_acc, RunError},
    },
    create_with_args,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Outbox;

impl Outbox {
    fn create(args: &str) -> Option<Self> {
        if args.is_empty() {
            Some(Self)
        } else {
            None
        }
    }
}

impl Command for Outbox {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let value = get_acc(game_state.acc)?;

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
        Ok(())
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(OutboxFactory)
    }
}

pub struct OutboxFactory;

impl CommandFactory for OutboxFactory {
    fn command(&self) -> &'static str {
        "OUTBOX"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(Outbox, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:outbox
    #[test]
    fn create_succeeds() {
        let command = Outbox::create("").unwrap();
        assert_eq!(Outbox, command);
    }

    #[test]
    fn create_fails() {
        let command = Outbox::create("a");
        assert!(command.is_none());

        let command = Outbox::create("1");
        assert!(command.is_none());

        let command = Outbox::create(" ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("OUTBOX", OutboxFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = OutboxFactory.create("");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = OutboxFactory.create("a");
        assert!(command.is_none());

        let command = OutboxFactory.create("1");
        assert!(command.is_none());

        let command = OutboxFactory.create(" ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
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

        Outbox
            .execute(&Default::default(), &mut game_state)
            .unwrap();
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

    #[test]
    fn next_test() {
        let game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        assert_eq!(1, Outbox.next(&Default::default(), &game_state).unwrap());
    }

    #[test]
    fn requires_index_test() {
        assert!(Outbox.requires_index().is_none());
    }

    #[test]
    fn requires_label_test() {
        assert!(Outbox.requires_label().is_none());
    }

    #[test]
    fn factory_test() {
        assert_eq!("OUTBOX", Outbox.factory().command());
    }
    // endregion
}
