use std::{env, fs};

use hrm::compiler::compile::compile_code;
use hrm::game::problem::Problem;
use hrm::model::problem_definition::ProblemDefinition;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing problem and/or solution files");
    }

    let problem = fs::read_to_string(&args[1]).unwrap();
    let solution = fs::read_to_string(&args[2]).unwrap();

    let problem: ProblemDefinition = serde_json::from_str(&problem).unwrap();
    let problem: Problem = problem.into();
    let program = compile_code(&solution).unwrap();

    program.validate(&problem).unwrap();
    let score = program.run(&problem).unwrap();
    println!("score = {:?}", score);
}