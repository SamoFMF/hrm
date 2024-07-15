use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory},
        game_state::GameState,
        program::{Program, RunError},
    },
    compiler::compile::compile_label,
    create_with_args,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Jump(pub String);

impl Jump {
    fn create(args: &str) -> Option<Self> {
        compile_label(args).map(Jump)
    }
}

impl Command for Jump {
    fn execute(&self, _program: &Program, _game_state: &mut GameState) -> Result<(), RunError> {
        Ok(())
    }

    /// Jump To
    ///
    /// # Panics
    ///
    /// See [Program::get_label].
    fn next(&self, program: &Program, _game_state: &GameState) -> Option<usize> {
        Some(program.get_label(&self.0))
    }

    fn requires_label(&self) -> Option<&str> {
        Some(&self.0)
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(JumpFactory)
    }
}

pub struct JumpFactory;

impl CommandFactory for JumpFactory {
    fn command(&self) -> &'static str {
        "JUMP"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(Jump, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::code::program::ProgramBuilder;
    use crate::game::value::Value;

    use super::*;

    // region:jump
    #[test]
    fn create_succeeds() {
        let command = Jump::create("a").unwrap();
        assert_eq!(Jump(String::from("a")), command);
    }

    #[test]
    fn create_fails() {
        let command = Jump::create("");
        assert!(command.is_none());

        let command = Jump::create("1");
        assert!(command.is_none());

        let command = Jump::create("a1");
        assert!(command.is_none());

        let command = Jump::create(" ");
        assert!(command.is_none());

        let command = Jump::create(" a ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("JUMP", JumpFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = JumpFactory.create("a");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = JumpFactory.create("");
        assert!(command.is_none());

        let command = JumpFactory.create("1");
        assert!(command.is_none());

        let command = JumpFactory.create("a1");
        assert!(command.is_none());

        let command = JumpFactory.create(" ");
        assert!(command.is_none());

        let command = JumpFactory.create(" a ");
        assert!(command.is_none());
    }
    // endregion

    // region:command
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

        let i_next = Jump(String::from("a")).next(&program, &game_state).unwrap();
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

    #[test]
    fn factory_test() {
        assert_eq!("JUMP", Jump(String::from("a")).factory().command());
    }
    // endregion
}
