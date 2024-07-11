use crate::{
    code::{
        commands::{command::CommandNew, CommandValue},
        game_state::GameState,
        program::{try_get_from_memory, try_get_index, Program, RunError},
    },
    compiler::compile::try_compile_command_value,
};

const COMMAND: &str = "COPYFROM";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CopyFrom(pub CommandValue);

impl CommandNew for CopyFrom {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        CopyFrom::command_static()
    }

    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if command != COMMAND {
            return None;
        }

        try_compile_command_value(args).map(|command_value| CopyFrom(command_value))
    }

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let index = try_get_index(&self.0, &game_state.memory)?;
        game_state.acc = Some(try_get_from_memory(game_state.memory[index])?);

        Ok(())
    }

    fn requires_index(&self) -> Option<usize> {
        match self.0 {
            CommandValue::Value(_) => None,
            CommandValue::Index(idx) => Some(idx),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn command_static_test() {
        assert_eq!(COMMAND, CopyFrom::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, CopyFrom(CommandValue::Value(5)).command());
    }

    #[test]
    fn create_succeeds() {
        let command = CopyFrom::create("COPYFROM", "42").unwrap();
        assert_eq!(CopyFrom(CommandValue::Value(42)), command);

        let command = CopyFrom::create("COPYFROM", "[42]").unwrap();
        assert_eq!(CopyFrom(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = CopyFrom::create("INBOX", "");
        assert_eq!(None, command);

        let command = CopyFrom::create("COPYTO", "42");
        assert_eq!(None, command);

        let command = CopyFrom::create("COPYFROM", "");
        assert_eq!(None, command);

        let command = CopyFrom::create("COPYFROM", "a");
        assert_eq!(None, command);

        let command = CopyFrom::create("COPYFROM", "a1");
        assert_eq!(None, command);

        let command = CopyFrom::create("COPYFROM", " ");
        assert_eq!(None, command);

        let command = CopyFrom::create("COPYFROM", " 1 ");
        assert_eq!(None, command);
    }

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
        assert_eq!(RunError::EmptyMemoryNew, result);
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
        assert_eq!(RunError::EmptyMemoryNew, result);
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
            CopyFrom(CommandValue::Value(1)).next(&Default::default(), &game_state)
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
}
