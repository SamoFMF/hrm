use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory, CommandValue},
        game_state::GameState,
        program::{get_from_memory, get_index, Program, RunError},
    },
    compiler::compile::compile_command_value,
    create_with_args,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CopyFrom(pub CommandValue);

impl CopyFrom {
    fn create(args: &str) -> Option<Self> {
        compile_command_value(args).map(CopyFrom)
    }
}

impl Command for CopyFrom {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let index = get_index(&self.0, &game_state.memory)?;
        game_state.acc = Some(get_from_memory(game_state.memory[index])?);

        Ok(())
    }

    fn requires_index(&self) -> Option<usize> {
        match self.0 {
            CommandValue::Value(_) => None,
            CommandValue::Index(idx) => Some(idx),
        }
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(CopyFromFactory)
    }
}

pub struct CopyFromFactory;

impl CommandFactory for CopyFromFactory {
    fn command(&self) -> &'static str {
        "COPYFROM"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(CopyFrom, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:copyfrom
    #[test]
    fn create_succeeds() {
        let command = CopyFrom::create("42").unwrap();
        assert_eq!(CopyFrom(CommandValue::Value(42)), command);

        let command = CopyFrom::create("[42]").unwrap();
        assert_eq!(CopyFrom(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = CopyFrom::create("");
        assert!(command.is_none());

        let command = CopyFrom::create("");
        assert!(command.is_none());

        let command = CopyFrom::create("a");
        assert!(command.is_none());

        let command = CopyFrom::create("a1");
        assert!(command.is_none());

        let command = CopyFrom::create(" ");
        assert!(command.is_none());

        let command = CopyFrom::create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("COPYFROM", CopyFromFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = CopyFromFactory.create("42");
        assert!(command.is_some());

        let command = CopyFromFactory.create("[42]");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = CopyFromFactory.create("");
        assert!(command.is_none());

        let command = CopyFromFactory.create("");
        assert!(command.is_none());

        let command = CopyFromFactory.create("a");
        assert!(command.is_none());

        let command = CopyFromFactory.create("a1");
        assert!(command.is_none());

        let command = CopyFromFactory.create(" ");
        assert!(command.is_none());

        let command = CopyFromFactory.create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(1)), Some(Value::Char('A'))],
            acc: Some(Value::Int(1)),
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        CopyFrom(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(1), game_state.acc.unwrap());

        CopyFrom(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Char('A'), game_state.acc.unwrap());
    }

    #[test]
    fn execute_empty_memory() {
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

        let result = CopyFrom(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::EmptyMemory, result);
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

        let result = CopyFrom(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::IndexOutOfRange(Value::Int(5)), result);

        let result = CopyFrom(CommandValue::Index(1))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::CharIndex(Value::Char('A')), result);

        let result = CopyFrom(CommandValue::Index(2))
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
            CopyFrom(CommandValue::Value(1))
                .next(&Default::default(), &game_state)
                .unwrap()
        );
    }

    #[test]
    fn requires_index_test() {
        let command = CopyFrom(CommandValue::Value(42));
        assert!(command.requires_index().is_none());

        let command = CopyFrom(CommandValue::Index(42));
        assert_eq!(42, command.requires_index().unwrap());
    }

    #[test]
    fn requires_label_test() {
        assert!(CopyFrom(CommandValue::Value(42)).requires_label().is_none());
        assert!(CopyFrom(CommandValue::Index(42)).requires_label().is_none());
    }

    #[test]
    fn factory_test() {
        assert_eq!(
            "COPYFROM",
            CopyFrom(CommandValue::Value(42)).factory().command()
        );
        assert_eq!(
            "COPYFROM",
            CopyFrom(CommandValue::Index(42)).factory().command()
        );
    }
    // endregion
}
