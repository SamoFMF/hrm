use std::collections::{HashMap, HashSet};

use crate::code::commands::ALL_COMMANDS;
use crate::game::value::Value;

#[derive(Debug)]
pub struct Problem {
    pub title: String,
    pub description: String,
    ios: Vec<ProblemIO>,
    memory: Vec<Option<Value>>,
    available_commands: HashSet<String>,
}

impl Problem {
    pub fn new(
        title: String,
        description: String,
        ios: Vec<ProblemIO>,
        memory: Vec<Option<Value>>,
        available_commands: HashSet<String>,
    ) -> Self {
        Self {
            title,
            description,
            ios,
            memory,
            available_commands,
        }
    }

    pub fn get_ios(&self) -> &Vec<ProblemIO> {
        &self.ios
    }

    pub fn get_memory(&self) -> &Vec<Option<Value>> {
        &self.memory
    }

    pub fn is_command_available(&self, command: &str) -> bool {
        self.available_commands.contains(command)
    }
}

pub struct ProblemBuilder {
    title: String,
    description: String,
    ios: Vec<ProblemIO>,
    memory: HashMap<usize, Value>,
    memory_dim: Option<usize>,
    available_commands: HashSet<String>,
}

impl Default for ProblemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProblemBuilder {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            ios: vec![],
            memory: Default::default(),
            memory_dim: None,
            available_commands: Default::default(),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn add_io(mut self, problem_io: ProblemIO) -> Self {
        self.ios.push(problem_io);
        self
    }

    pub fn memory_dim(mut self, dim: usize) -> Self {
        self.memory_dim = Some(dim);
        self
    }

    pub fn add_memory_slot(mut self, slot: usize, value: Value) -> Self {
        self.memory.insert(slot, value);
        self
    }

    pub fn enable_all_commands(mut self) -> Self {
        self.available_commands =
            HashSet::from_iter(ALL_COMMANDS.iter().map(|command| command.to_string()));
        self
    }

    pub fn enable_command(mut self, command: String) -> Self {
        if ALL_COMMANDS.contains(&command.as_str()) {
            self.available_commands.insert(command);
        }
        self
    }

    pub fn disable_command(mut self, command: &str) -> Self {
        self.available_commands.remove(command);
        self
    }

    pub fn build(self) -> Problem {
        let mut memory = match self.memory_dim {
            Some(memory_dim) => vec![None; memory_dim],
            None => vec![],
        };

        for (i, value) in self.memory {
            if i < memory.len() {
                memory[i] = Some(value);
            }
        }

        Problem::new(
            self.title,
            self.description,
            self.ios,
            memory,
            self.available_commands,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ProblemIO {
    pub input: Vec<Value>,
    pub output: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // region:ProblemBuilder
    #[test]
    fn enable_all_commands_test() {
        let problem = ProblemBuilder::new()
            .add_io(ProblemIO {
                input: vec![],
                output: vec![],
            })
            .memory_dim(0)
            .enable_all_commands()
            .build();

        assert_eq!(ALL_COMMANDS.len(), problem.available_commands.len());
        for command in ALL_COMMANDS {
            assert!(problem.is_command_available(command));
        }
    }

    #[test]
    fn enable_command_test() {
        let available_command = String::from("SUB");
        let problem = ProblemBuilder::new()
            .add_io(ProblemIO {
                input: vec![],
                output: vec![],
            })
            .memory_dim(0)
            .enable_command(available_command.clone())
            .build();

        assert!(problem.is_command_available(&available_command));

        ALL_COMMANDS
            .iter()
            .filter(|command| **command != available_command)
            .for_each(|command| assert!(!problem.is_command_available(*command)));
    }

    #[test]
    fn disable_command_test() {
        let unavailable_command = "SUB";
        let problem = ProblemBuilder::new()
            .add_io(ProblemIO {
                input: vec![],
                output: vec![],
            })
            .memory_dim(0)
            .enable_all_commands()
            .disable_command(unavailable_command)
            .build();

        assert!(!problem.is_command_available(unavailable_command));

        ALL_COMMANDS
            .iter()
            .filter(|command| **command != unavailable_command)
            .for_each(|command| assert!(problem.is_command_available(*command)));
    }
    // endregion
}
