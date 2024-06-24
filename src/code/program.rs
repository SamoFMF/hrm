use std::collections::HashMap;

use crate::code::commands::Command;

#[derive(Debug)]
pub struct Program {
    // todo: add comments & labels - verify them
    commands: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl Program {
    fn is_valid() -> bool {
        todo!()
    }

    fn run() {
        todo!()
    }
}

pub struct ProgramBuilder {
    commands: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            labels: HashMap::new(),
        }
    }

    pub fn build(mut self) -> Program {
        self.commands.push(Command::End);
        Program {
            commands: self.commands,
            labels: self.labels,
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.insert(label, self.commands.len());
    }
}