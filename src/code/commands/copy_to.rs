use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory, CommandValue},
        game_state::GameState,
        program::{get_acc, get_index, Program, RunError},
    },
    compiler::compile::compile_command_value,
    create_with_args,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CopyTo(pub CommandValue);

impl CopyTo {
    fn create(args: &str) -> Option<Self> {
        compile_command_value(args).map(CopyTo)
    }
}

impl Command for CopyTo {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let value = get_acc(game_state.acc)?;
        let index = get_index(&self.0, &game_state.memory)?;
        game_state.memory[index] = Some(value);

        Ok(())
    }

    fn requires_index(&self) -> Option<usize> {
        match self.0 {
            CommandValue::Value(_) => None,
            CommandValue::Index(idx) => Some(idx),
        }
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(CopyToFactory)
    }
}

pub struct CopyToFactory;

impl CommandFactory for CopyToFactory {
    fn command(&self) -> &'static str {
        "COPYTO"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(CopyTo, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:copyto
    #[test]
    fn create_succeeds() {
        let command = CopyTo::create("42").unwrap();
        assert_eq!(CopyTo(CommandValue::Value(42)), command);

        let command = CopyTo::create("[42]").unwrap();
        assert_eq!(CopyTo(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = CopyTo::create("");
        assert!(command.is_none());

        let command = CopyTo::create("");
        assert!(command.is_none());

        let command = CopyTo::create("a");
        assert!(command.is_none());

        let command = CopyTo::create("a1");
        assert!(command.is_none());

        let command = CopyTo::create(" ");
        assert!(command.is_none());

        let command = CopyTo::create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("COPYTO", CopyToFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = CopyToFactory.create("42");
        assert!(command.is_some());

        let command = CopyToFactory.create("[42]");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = CopyToFactory.create("");
        assert!(command.is_none());

        let command = CopyToFactory.create("");
        assert!(command.is_none());

        let command = CopyToFactory.create("a");
        assert!(command.is_none());

        let command = CopyToFactory.create("a1");
        assert!(command.is_none());

        let command = CopyToFactory.create(" ");
        assert!(command.is_none());

        let command = CopyToFactory.create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![None, None],
            acc: Some(Value::Int(1)),
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        CopyTo(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(1), game_state.memory[0].unwrap());
        assert_eq!(None, game_state.memory[1]);

        CopyTo(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(1), game_state.memory[0].unwrap());
        assert_eq!(Value::Int(1), game_state.memory[1].unwrap());
    }

    #[test]
    fn execute_no_acc() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![None],
            acc: None,
            i_input: 1,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let result = CopyTo(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::EmptyAcc, result);
    }

    #[test]
    fn execute_bad_index() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(5)), Some(Value::Char('A')), None],
            acc: Some(Value::Int(1)),
            i_input: 1,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let result = CopyTo(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::IndexOutOfRange(Value::Int(5)), result);

        let result = CopyTo(CommandValue::Index(1))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::CharIndex(Value::Char('A')), result);

        let result = CopyTo(CommandValue::Index(2))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::EmptyMemory, result);
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

        assert_eq!(
            1,
            CopyTo(CommandValue::Value(1)).next(&Default::default(), &game_state)
        );
    }

    #[test]
    fn requires_index_test() {
        let command = CopyTo(CommandValue::Value(42));
        assert!(command.requires_index().is_none());

        let command = CopyTo(CommandValue::Index(42));
        assert_eq!(42, command.requires_index().unwrap());
    }

    #[test]
    fn requires_label_test() {
        assert!(CopyTo(CommandValue::Value(42)).requires_label().is_none());
        assert!(CopyTo(CommandValue::Index(42)).requires_label().is_none());
    }

    #[test]
    fn factory_test() {
        assert_eq!(
            "COPYTO",
            CopyTo(CommandValue::Value(42)).factory().command()
        );
        assert_eq!(
            "COPYTO",
            CopyTo(CommandValue::Index(42)).factory().command()
        );
    }
    // endregion
}
