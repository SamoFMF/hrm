use crate::{
    code::{
        commands::{Command, CommandValue},
        game_state::GameState,
        program::{get_acc, get_from_memory, get_index, Program, RunError},
    },
    compiler::compile::compile_command_value,
};

const COMMAND: &str = "SUB";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sub(pub CommandValue);

impl Command for Sub {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        Sub::command_static()
    }

    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if command != COMMAND {
            return None;
        }

        compile_command_value(args).map(|command_value| Sub(command_value))
    }

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let value = get_acc(game_state.acc)?;
        let index = get_index(&self.0, &game_state.memory)?;
        let to_sub = get_from_memory(game_state.memory[index])?;
        let diff = value.sub(to_sub).ok_or(RunError::SubNew)?;
        game_state.acc = Some(diff);
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
        assert_eq!(COMMAND, Sub::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, Sub(CommandValue::Value(5)).command());
    }

    #[test]
    fn create_succeeds() {
        let command = Sub::create("SUB", "42").unwrap();
        assert_eq!(Sub(CommandValue::Value(42)), command);

        let command = Sub::create("SUB", "[42]").unwrap();
        assert_eq!(Sub(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = Sub::create("INBOX", "");
        assert_eq!(None, command);

        let command = Sub::create("ADD", "42");
        assert_eq!(None, command);

        let command = Sub::create("SUB", "");
        assert_eq!(None, command);

        let command = Sub::create("SUB", "a");
        assert_eq!(None, command);

        let command = Sub::create("SUB", "a1");
        assert_eq!(None, command);

        let command = Sub::create("SUB", " ");
        assert_eq!(None, command);

        let command = Sub::create("SUB", " 1 ");
        assert_eq!(None, command);
    }

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
            Sub(CommandValue::Value(1)).next(&Default::default(), &game_state)
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
}
