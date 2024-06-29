use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::problem::{Problem, ProblemBuilder, ProblemIO};
use crate::game::value::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDefinition {
    pub ios: Vec<ProblemDefinitionIO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<ProblemDefinitionMemory>,
    pub commands: Vec<String>,
}

impl Into<Problem> for ProblemDefinition {
    fn into(self) -> Problem {
        let mut builder = ProblemBuilder::new();
        for problem_io in self.ios {
            builder = builder.add_io(problem_io.into());
        }

        if let Some(memory) = self.memory {
            match memory {
                ProblemDefinitionMemory { full: Some(full), partial: _ } => {
                    builder = builder.memory_dim(full.len());
                    for (i, value) in full.iter().enumerate() {
                        if let Some(value) = *value {
                            builder = builder.add_memory_slot(i, value);
                        }
                    }
                }
                ProblemDefinitionMemory { full: None, partial: Some(partial) } => {
                    builder = builder.memory_dim(partial.dim);
                    for (i, value) in partial.values {
                        builder = builder.add_memory_slot(i, value);
                    }
                }
                _ => {}
            }
        }

        for command in self.commands {
            builder = builder.enable_command(command);
        }

        builder.build()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDefinitionIO {
    pub input: Vec<Value>,
    pub output: Vec<Value>,
}

impl Into<ProblemIO> for ProblemDefinitionIO {
    fn into(self) -> ProblemIO {
        ProblemIO {
            input: self.input,
            output: self.output,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDefinitionMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<Vec<Option<Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<PartialMemory>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialMemory {
    pub dim: usize,
    pub values: HashMap<usize, Value>,
}

#[cfg(test)]
mod tests {
    use crate::game::value::Value;

    use super::*;

    #[test]
    fn serde_problem_definition() {
        let problem_definition = create_problem_definition();

        let serialized = serde_json::to_string(&problem_definition).unwrap();
        let deserialized: ProblemDefinition = serde_json::from_str(&serialized).unwrap();

        assert_eq!(problem_definition, deserialized);
    }

    #[test]
    fn into_problem() {
        let problem_definition = create_problem_definition();
        let problem: Problem = problem_definition.into();

        assert_eq!(1, problem.get_ios().len());
        assert_eq!(2, problem.get_memory().len());
    }

    #[test]
    fn deserialize_problem_definition() {
        let json = "\
        {
            \"ios\": [
                {
                  \"input\": [1, 2, 3],
                  \"output\": [1, 2, 3]
                }
          ],
          \"commands\": [\"INBOX\", \"OUTBOX\"]
        }";

        let problem_definition: ProblemDefinition = serde_json::from_str(json).unwrap();

        assert_eq!(1, problem_definition.ios.len());
        assert_eq!(None, problem_definition.memory);
        assert_eq!(2, problem_definition.commands.len())
    }

    fn create_problem_definition() -> ProblemDefinition {
        let problem_io = ProblemDefinitionIO {
            input: vec![Value::Int(-5), Value::Char('A')],
            output: vec![Value::Int(123), Value::Char('0')],
        };

        let memory = ProblemDefinitionMemory {
            full: Some(vec![None, Some(Value::Int(1))]),
            partial: None,
        };

        let commands = vec![String::from("INBOX"), String::from("OUTBOX")];

        ProblemDefinition {
            ios: vec![problem_io],
            memory: Some(memory),
            commands,
        }
    }
}
