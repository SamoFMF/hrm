use crate::{
    code::{
        commands::{AnyCommand, Command, CommandFactory},
        game_state::GameState,
        program::{get_acc, Program, RunError},
    },
    compiler::compile::compile_label,
    create_with_args,
};

#[derive(Debug, Clone, PartialEq)]
pub struct JumpZero(pub String);

impl JumpZero {
    fn create(args: &str) -> Option<Self> {
        compile_label(args).map(JumpZero)
    }
}

impl Command for JumpZero {
    fn execute(&self, _program: &Program, game_state: &mut GameState) -> Result<(), RunError> {
        get_acc(game_state.acc).map(|_| ())
    }

    /// Jump To If Zero
    ///
    /// Jumps to label if [GameState]`.acc` equals `0`, else increments [GameState]`.i_command`.
    ///
    /// # Panics
    ///
    /// Can be caused by:
    /// - if [GameState]`.acc` is [None] - this is prevented by calling [JumpZero::execute] first
    /// - see [Program::get_label].
    fn next(&self, program: &Program, game_state: &GameState) -> usize {
        if get_acc(game_state.acc).unwrap() == 0 {
            program.get_label(&self.0)
        } else {
            game_state.i_command + 1
        }
    }

    fn requires_label(&self) -> Option<&str> {
        Some(&self.0)
    }

    fn factory(&self) -> Box<dyn CommandFactory> {
        Box::new(JumpZeroFactory)
    }
}

pub struct JumpZeroFactory;

impl CommandFactory for JumpZeroFactory {
    fn command(&self) -> &'static str {
        "JUMPZ"
    }

    fn create(&self, args: &str) -> Option<AnyCommand> {
        create_with_args!(JumpZero, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::code::program::ProgramBuilder;
    use crate::game::value::Value;

    use super::*;

    // region:jumpzero
    #[test]
    fn create_succeeds() {
        let command = JumpZero::create("a").unwrap();
        assert_eq!(JumpZero(String::from("a")), command);
    }

    #[test]
    fn create_fails() {
        let command = JumpZero::create("");
        assert!(command.is_none());

        let command = JumpZero::create("");
        assert!(command.is_none());

        let command = JumpZero::create("1");
        assert!(command.is_none());

        let command = JumpZero::create("a1");
        assert!(command.is_none());

        let command = JumpZero::create(" ");
        assert!(command.is_none());

        let command = JumpZero::create(" a ");
        assert!(command.is_none());
    }
    // endregion

    // region:factory
    #[test]
    fn command_test() {
        assert_eq!("JUMPZ", JumpZeroFactory.command());
    }

    #[test]
    fn factory_create_succeeds() {
        let command = JumpZeroFactory.create("a");
        assert!(command.is_some());
    }

    #[test]
    fn factory_create_fails() {
        let command = JumpZeroFactory.create("");
        assert!(command.is_none());

        let command = JumpZeroFactory.create("");
        assert!(command.is_none());

        let command = JumpZeroFactory.create("1");
        assert!(command.is_none());

        let command = JumpZeroFactory.create("a1");
        assert!(command.is_none());

        let command = JumpZeroFactory.create(" ");
        assert!(command.is_none());

        let command = JumpZeroFactory.create(" a ");
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
            i_command: 5,
            speed: 0,
        };

        let program = ProgramBuilder::new().add_label(String::from("a")).build();

        JumpZero(String::from("a"))
            .execute(&program, &mut game_state)
            .unwrap();

        game_state.acc = Some(Value::Char('A'));
        JumpZero(String::from("a"))
            .execute(&program, &mut game_state)
            .unwrap();
    }

    #[test]
    fn execute_no_acc() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![],
            acc: None,
            i_input: 0,
            i_output: 0,
            i_command: 0,
            speed: 0,
        };

        let program = ProgramBuilder::new().add_label(String::from("a")).build();

        let result = JumpZero(String::from("a"))
            .execute(&program, &mut game_state)
            .unwrap_err();
        assert_eq!(RunError::EmptyAcc, result);
    }

    #[test]
    fn next_test() {
        let mut game_state = GameState {
            input: &vec![],
            output: &vec![],
            memory: vec![Some(Value::Int(1)), Some(Value::Int(42))],
            acc: Some(Value::Int(0)),
            i_input: 0,
            i_output: 0,
            i_command: 5,
            speed: 0,
        };

        let program = ProgramBuilder::new().add_label(String::from("a")).build();

        let i_next = JumpZero(String::from("a")).next(&program, &game_state);
        assert_eq!(0, i_next);

        game_state.acc = Some(Value::Int(1));
        let i_next = JumpZero(String::from("a")).next(&program, &game_state);
        assert_eq!(6, i_next);

        game_state.acc = Some(Value::Int(-1));
        let i_next = JumpZero(String::from("a")).next(&program, &game_state);
        assert_eq!(6, i_next);

        game_state.acc = Some(Value::Char('A'));
        let i_next = JumpZero(String::from("a")).next(&program, &game_state);
        assert_eq!(6, i_next);
    }

    #[test]
    fn requires_index_test() {
        assert!(JumpZero(String::new()).requires_index().is_none());
    }

    #[test]
    fn requires_label_test() {
        let command = JumpZero(String::from("a"));
        assert_eq!("a", command.requires_label().unwrap());
    }

    #[test]
    fn factory_test() {
        assert_eq!("JUMPZ", JumpZero(String::from("a")).factory().command());
    }
    // endregion
}
