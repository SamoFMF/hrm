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
pub struct Add(pub CommandValue);

impl Add {
    fn create(args: &str) -> Option<Self> {
        compile_command_value(args).map(Add)
    }
}

impl Command for Add {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let value = get_acc(game_state.acc)?;
        let index = get_index(&self.0, &game_state.memory)?;
        let to_add = get_from_memory(game_state.memory[index])?;
        let sum = value.hrm_add(to_add).ok_or(RunError::Add)?;
        game_state.acc = Some(sum);
        Ok(())
    }

    fn requires_index(&self) -> Option<usize> {
        match self.0 {
            CommandValue::Value(_) => None,
            CommandValue::Index(idx) => Some(idx),
        }
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(AddFactory)
    }
}

pub struct AddFactory;

impl CommandFactory for AddFactory {
    fn command(&self) -> &'static str {
        "ADD"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        // Add::create(args).map(|add| Box::new(add) as AnyCommand)
        create_with_args!(Add, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:add
    #[test]
    fn create_succeeds() {
        let command = Add::create("42").unwrap();
        assert_eq!(Add(CommandValue::Value(42)), command);

        let command = Add::create("[42]").unwrap();
        assert_eq!(Add(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = Add::create("");
        assert!(command.is_none());

        let command = Add::create("");
        assert!(command.is_none());

        let command = Add::create("a");
        assert!(command.is_none());

        let command = Add::create("a1");
        assert!(command.is_none());

        let command = Add::create(" ");
        assert!(command.is_none());

        let command = Add::create(" 1 ");
        assert!(command.is_none());
    }
    // endregion
    #[test]
    fn command_test() {
        assert_eq!("ADD", AddFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = AddFactory.create("42");
        assert!(command.is_some());

        let command = AddFactory.create("[42]");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = AddFactory.create("");
        assert!(command.is_none());

        let command = AddFactory.create("");
        assert!(command.is_none());

        let command = AddFactory.create("a");
        assert!(command.is_none());

        let command = AddFactory.create("a1");
        assert!(command.is_none());

        let command = AddFactory.create(" ");
        assert!(command.is_none());

        let command = AddFactory.create(" 1 ");
        assert!(command.is_none());
    }
    // region:factory

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

        Add(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(2), game_state.acc.unwrap());

        Add(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(44), game_state.acc.unwrap());
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

        let result = Add(CommandValue::Value(0))
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

        let result = Add(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::IndexOutOfRange(Value::Int(5)), result);

        let result = Add(CommandValue::Index(1))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::CharIndex(Value::Char('A')), result);

        let result = Add(CommandValue::Index(2))
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
            Add(CommandValue::Value(1))
                .next(&Default::default(), &game_state)
                .unwrap()
        );
    }

    #[test]
    fn requires_index_test() {
        let command = Add(CommandValue::Value(42));
        assert!(command.requires_index().is_none());

        let command = Add(CommandValue::Index(42));
        assert_eq!(42, command.requires_index().unwrap());
    }

    #[test]
    fn requires_label_test() {
        assert!(Add(CommandValue::Value(42)).requires_label().is_none());
        assert!(Add(CommandValue::Index(42)).requires_label().is_none());
    }

    #[test]
    fn factory_test() {
        assert_eq!("ADD", Add(CommandValue::Value(42)).factory().command());
        assert_eq!("ADD", Add(CommandValue::Index(42)).factory().command());
    }
    // endregion
}
