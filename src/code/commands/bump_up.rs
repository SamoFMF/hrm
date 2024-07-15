use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory, CommandValue},
        game_state::GameState,
        program::{
            Program, RunError, {get_from_memory, get_index},
        },
    },
    compiler::compile::compile_command_value,
    create_with_args,
    game::value::Value,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BumpUp(pub CommandValue);

impl BumpUp {
    fn create(args: &str) -> Option<Self> {
        compile_command_value(args).map(BumpUp)
    }
}

impl Command for BumpUp {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let index = get_index(&self.0, &game_state.memory)?;
        let to_bump = get_from_memory(game_state.memory[index])?;
        let bumped = to_bump.hrm_add(Value::Int(1)).ok_or(RunError::Add)?;
        game_state.memory[index] = Some(bumped);
        game_state.acc = Some(bumped);
        Ok(())
    }

    fn requires_index(&self) -> Option<usize> {
        match self.0 {
            CommandValue::Value(_) => None,
            CommandValue::Index(idx) => Some(idx),
        }
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(BumpUpFactory)
    }
}

pub struct BumpUpFactory;

impl CommandFactory for BumpUpFactory {
    fn command(&self) -> &'static str {
        "BUMPUP"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(BumpUp, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    // region:bumpup
    #[test]
    fn create_succeeds() {
        let command = BumpUp::create("42").unwrap();
        assert_eq!(BumpUp(CommandValue::Value(42)), command);

        let command = BumpUp::create("[42]").unwrap();
        assert_eq!(BumpUp(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = BumpUp::create("");
        assert!(command.is_none());

        let command = BumpUp::create("");
        assert!(command.is_none());

        let command = BumpUp::create("a");
        assert!(command.is_none());

        let command = BumpUp::create("a1");
        assert!(command.is_none());

        let command = BumpUp::create(" ");
        assert!(command.is_none());

        let command = BumpUp::create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("BUMPUP", BumpUpFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = BumpUpFactory.create("42");
        assert!(command.is_some());

        let command = BumpUpFactory.create("[42]");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = BumpUpFactory.create("");
        assert!(command.is_none());

        let command = BumpUpFactory.create("");
        assert!(command.is_none());

        let command = BumpUpFactory.create("a");
        assert!(command.is_none());

        let command = BumpUpFactory.create("a1");
        assert!(command.is_none());

        let command = BumpUpFactory.create(" ");
        assert!(command.is_none());

        let command = BumpUpFactory.create(" 1 ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(0)), Some(Value::Int(42))],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        BumpUp(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(1), game_state.acc.unwrap());
        assert_eq!(Value::Int(1), game_state.memory[0].unwrap());
        assert_eq!(Value::Int(42), game_state.memory[1].unwrap());

        BumpUp(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(43), game_state.acc.unwrap());
        assert_eq!(Value::Int(1), game_state.memory[0].unwrap());
        assert_eq!(Value::Int(43), game_state.memory[1].unwrap());
    }

    #[test]
    fn execute_char() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Char('A'))],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let result = BumpUp(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::Add, result);
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

        let result = BumpUp(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::IndexOutOfRange(Value::Int(5)), result);

        let result = BumpUp(CommandValue::Index(1))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::CharIndex(Value::Char('A')), result);

        let result = BumpUp(CommandValue::Index(2))
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
            BumpUp(CommandValue::Value(1))
                .next(&Default::default(), &game_state)
                .unwrap()
        );
    }

    #[test]
    fn requires_index_test() {
        let command = BumpUp(CommandValue::Value(42));
        assert!(command.requires_index().is_none());

        let command = BumpUp(CommandValue::Index(42));
        assert_eq!(42, command.requires_index().unwrap());
    }

    #[test]
    fn requires_label_test() {
        assert!(BumpUp(CommandValue::Value(42)).requires_label().is_none());
        assert!(BumpUp(CommandValue::Index(42)).requires_label().is_none());
    }

    #[test]
    fn factory_test() {
        assert_eq!(
            "BUMPUP",
            BumpUp(CommandValue::Value(42)).factory().command()
        );
        assert_eq!(
            "BUMPUP",
            BumpUp(CommandValue::Index(42)).factory().command()
        );
    }
    // endregion
}
