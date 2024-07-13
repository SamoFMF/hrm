use crate::{
    code::{
        commands::Command,
        game_state::GameState,
        program::{Program, RunError},
    },
    compiler::compile::compile_label,
};

const COMMAND: &str = "JUMP";

#[derive(Debug, Clone, PartialEq)]
pub struct Jump(pub String);

impl Command for Jump {
    fn command_static() -> &'static str
    where
        Self: Sized,
    {
        COMMAND
    }

    fn command(&self) -> &'static str {
        Jump::command_static()
    }

    fn create(command: &str, args: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if command != COMMAND {
            return None;
        }

        compile_label(args).map(|label| Jump(label))
    }

    fn execute(&self, _program: &Program, _game_state: &mut GameState) -> Result<(), RunError> {
        Ok(())
    }

    /// Jump To
    ///
    /// # Panics
    ///
    /// See [Program::get_label].
    fn next(&self, program: &Program, _game_state: &GameState) -> usize {
        program.get_label(&self.0)
    }

    fn requires_label(&self) -> Option<&str> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::code::program::ProgramBuilder;
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn command_static_test() {
        assert_eq!(COMMAND, Jump::command_static());
    }

    #[test]
    fn command_test() {
        assert_eq!(COMMAND, Jump(String::from("a")).command());
    }

    #[test]
    fn create_succeeds() {
        let command = Jump::create("JUMP", "a").unwrap();
        assert_eq!(Jump(String::from("a")), command);
    }

    #[test]
    fn create_fails() {
        let command = Jump::create("INBOX", "");
        assert_eq!(None, command);

        let command = Jump::create("JUMPZ", "a");
        assert_eq!(None, command);

        let command = Jump::create("JUMP", "");
        assert_eq!(None, command);

        let command = Jump::create("JUMP", "1");
        assert_eq!(None, command);

        let command = Jump::create("JUMP", "a1");
        assert_eq!(None, command);

        let command = Jump::create("JUMP", " ");
        assert_eq!(None, command);

        let command = Jump::create("JUMP", " a ");
        assert_eq!(None, command);
    }

    #[test]
    fn next_test() {
        let game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(1)), Some(Value::Int(42))],
            acc: Some(Value::Int(1)),
            i_input: 0,
            i_output: 0,
            i_command: 5,
            speed: 0,
        };

        let program = ProgramBuilder::new().add_label(String::from("a")).build();

        let i_next = Jump(String::from("a")).next(&program, &game_state);
        assert_eq!(0, i_next);
    }

    #[test]
    fn requires_index_test() {
        assert!(Jump(String::new()).requires_index().is_none());
    }

    #[test]
    fn requires_label_test() {
        let command = Jump(String::from("a"));
        assert_eq!("a", command.requires_label().unwrap());
    }
}
