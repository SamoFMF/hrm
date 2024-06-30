use crate::{
    code::{
        commands::{command::CommandNew, CommandValue},
        game_state::GameState,
        program::{try_get_acc, try_get_index, Program, RunError},
    },
    compiler::compile::try_compile_command_value,
};

const COMMAND: &str = "COPYTO";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CopyTo(CommandValue);

impl CommandNew for CopyTo {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        CopyTo::command_static()
    }

    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if command != COMMAND {
            return None;
        }

        try_compile_command_value(args).map(|command_value| CopyTo(command_value))
    }

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let value = try_get_acc(game_state.acc)?;
        let index = try_get_index(&self.0, &game_state.memory)?;
        game_state.memory[index] = Some(value);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn command_static_test() {
        assert_eq!(COMMAND, CopyTo::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, CopyTo(CommandValue::Value(5)).command());
    }

    #[test]
    fn create_succeeds() {
        let command = CopyTo::create("COPYTO", "42").unwrap();
        assert_eq!(CopyTo(CommandValue::Value(42)), command);

        let command = CopyTo::create("COPYTO", "[42]").unwrap();
        assert_eq!(CopyTo(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = CopyTo::create("INBOX", "");
        assert_eq!(None, command);

        let command = CopyTo::create("COPYFROM", "42");
        assert_eq!(None, command);

        let command = CopyTo::create("COPYTO", "");
        assert_eq!(None, command);

        let command = CopyTo::create("COPYTO", "a");
        assert_eq!(None, command);

        let command = CopyTo::create("COPYTO", "a1");
        assert_eq!(None, command);

        let command = CopyTo::create("COPYTO", " ");
        assert_eq!(None, command);

        let command = CopyTo::create("COPYTO", " 1 ");
        assert_eq!(None, command);
    }

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
        assert_eq!(RunError::EmptyAccNew, result);
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
            CopyTo(CommandValue::Value(1)).next(&Default::default(), &game_state)
        );
    }
}
