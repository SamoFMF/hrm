use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory, CommandValue},
        game_state::GameState,
        program::{get_acc, get_from_memory, get_index, Program, RunError},
    },
    compiler::compile::compile_command_value,
    create_with_args,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sub(pub CommandValue);

impl Sub {
    fn create(args: &str) -> Option<Self> {
        compile_command_value(args).map(Sub)
    }
}

impl Command for Sub {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let value = get_acc(game_state.acc)?;
        let index = get_index(&self.0, &game_state.memory)?;
        let to_sub = get_from_memory(game_state.memory[index])?;
        let diff = value.hrm_sub(to_sub).ok_or(RunError::Sub)?;
        game_state.acc = Some(diff);
        Ok(())
    }

    fn requires_index(&self) -> Option<usize> {
        match self.0 {
            CommandValue::Value(_) => None,
            CommandValue::Index(idx) => Some(idx),
        }
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(SubFactory)
    }
}

pub struct SubFactory;

impl CommandFactory for SubFactory {
    fn command(&self) -> &'static str {
        "SUB"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(Sub, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:sub
    #[test]
    fn create_succeeds() {
        let command = Sub::create("42").unwrap();
        assert_eq!(Sub(CommandValue::Value(42)), command);

        let command = Sub::create("[42]").unwrap();
        assert_eq!(Sub(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = Sub::create("");
        assert!(command.is_none());

        let command = Sub::create("a");
        assert!(command.is_none());

        let command = Sub::create("a1");
        assert!(command.is_none());

        let command = Sub::create(" ");
        assert!(command.is_none());

        let command = Sub::create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("SUB", SubFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = SubFactory.create("42");
        assert!(command.is_some());

        let command = SubFactory.create("[42]");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = SubFactory.create("");
        assert!(command.is_none());

        let command = SubFactory.create("a");
        assert!(command.is_none());

        let command = SubFactory.create("a1");
        assert!(command.is_none());

        let command = SubFactory.create(" ");
        assert!(command.is_none());

        let command = SubFactory.create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(1)), Some(Value::Int(42))],
            acc: Some(Value::Int(1)),
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        Sub(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(0), game_state.acc.unwrap());

        Sub(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(-42), game_state.acc.unwrap());
    }

    #[test]
    fn execute_no_acc() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(1)), Some(Value::Int(42))],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let result = Sub(CommandValue::Value(0))
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

        let result = Sub(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::IndexOutOfRange(Value::Int(5)), result);

        let result = Sub(CommandValue::Index(1))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::CharIndex(Value::Char('A')), result);

        let result = Sub(CommandValue::Index(2))
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
            Sub(CommandValue::Value(1))
                .next(&Default::default(), &game_state)
                .unwrap()
        );
    }

    #[test]
    fn requires_index_test() {
        let command = Sub(CommandValue::Value(42));
        assert!(command.requires_index().is_none());

        let command = Sub(CommandValue::Index(42));
        assert_eq!(42, command.requires_index().unwrap());
    }

    #[test]
    fn requires_label_test() {
        assert!(Sub(CommandValue::Value(42)).requires_label().is_none());
        assert!(Sub(CommandValue::Index(42)).requires_label().is_none());
    }

    #[test]
    fn factory_test() {
        assert_eq!("SUB", Sub(CommandValue::Value(42)).factory().command());
        assert_eq!("SUB", Sub(CommandValue::Index(42)).factory().command());
    }
    // endregion
}
