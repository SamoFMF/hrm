use crate::{
    code::{
        commands::{Command, CommandValue},
        game_state::GameState,
        program::{
            Program, RunError, {get_from_memory, get_index},
        },
    },
    compiler::compile::compile_command_value,
    game::value::Value,
};

const COMMAND: &str = "BUMPDN";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BumpDown(pub CommandValue);

impl Command for BumpDown {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        BumpDown::command_static()
    }

    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if command != COMMAND {
            return None;
        }

        compile_command_value(args).map(|command_value| BumpDown(command_value))
    }

    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        let index = get_index(&self.0, &game_state.memory)?;
        let to_bump = get_from_memory(game_state.memory[index])?;
        let bumped = to_bump.sub(Value::Int(1)).ok_or(RunError::SubNew)?;
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
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn command_static_test() {
        assert_eq!(COMMAND, BumpDown::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, BumpDown(CommandValue::Value(5)).command());
    }

    #[test]
    fn create_succeeds() {
        let command = BumpDown::create("BUMPDN", "42").unwrap();
        assert_eq!(BumpDown(CommandValue::Value(42)), command);

        let command = BumpDown::create("BUMPDN", "[42]").unwrap();
        assert_eq!(BumpDown(CommandValue::Index(42)), command);
    }

    #[test]
    fn create_fails() {
        let command = BumpDown::create("INBOX", "");
        assert_eq!(None, command);

        let command = BumpDown::create("BUMPUP", "42");
        assert_eq!(None, command);

        let command = BumpDown::create("BUMPDN", "");
        assert_eq!(None, command);

        let command = BumpDown::create("BUMPDN", "a");
        assert_eq!(None, command);

        let command = BumpDown::create("BUMPDN", "a1");
        assert_eq!(None, command);

        let command = BumpDown::create("BUMPDN", " ");
        assert_eq!(None, command);

        let command = BumpDown::create("BUMPDN", " 1 ");
        assert_eq!(None, command);
    }

    #[test]
    fn execute_succeeds() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(2)), Some(Value::Int(42))],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        BumpDown(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(1), game_state.acc.unwrap());
        assert_eq!(Value::Int(1), game_state.memory[0].unwrap());
        assert_eq!(Value::Int(42), game_state.memory[1].unwrap());

        BumpDown(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap();
        assert_eq!(Value::Int(41), game_state.acc.unwrap());
        assert_eq!(Value::Int(1), game_state.memory[0].unwrap());
        assert_eq!(Value::Int(41), game_state.memory[1].unwrap());
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

        let result = BumpDown(CommandValue::Value(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::SubNew, result);
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

        let result = BumpDown(CommandValue::Index(0))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::IndexOutOfRange(Value::Int(5)), result);

        let result = BumpDown(CommandValue::Index(1))
            .execute(&Default::default(), &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::CharIndex(Value::Char('A')), result);

        let result = BumpDown(CommandValue::Index(2))
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
            BumpDown(CommandValue::Value(1)).next(&Default::default(), &game_state)
        );
    }

    #[test]
    fn requires_index_test() {
        let command = BumpDown(CommandValue::Value(42));
        assert!(command.requires_index().is_none());

        let command = BumpDown(CommandValue::Index(42));
        assert_eq!(42, command.requires_index().unwrap());
    }

    #[test]
    fn requires_label_test() {
        assert!(BumpDown(CommandValue::Value(42)).requires_label().is_none());
        assert!(BumpDown(CommandValue::Index(42)).requires_label().is_none());
    }
}
