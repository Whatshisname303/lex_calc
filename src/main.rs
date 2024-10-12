use std::error::Error;
use std::io::{self, stdin, stdout, Write};
use std::collections::HashMap;
use std::f64::consts;
use std::process;

mod tokens;
mod tree_builder;

type Number = f64;

#[derive(Debug)]
enum MathType {
    Number(Number),
    Vector(Vec<Number>),
    Matix(Vec<Vec<Number>>),
}

#[derive(Debug)]
enum TrigMode {
    Rad,
    Deg,
}

// should probably have a constructor that reads these from config
struct Environment {
    user_vars: HashMap<String, MathType>,
    user_functions: HashMap<String, i32>, // need to define actual function type at some point
    trig_mode: TrigMode,
    digit_cap: u8,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            user_vars: HashMap::from([
                ("pi".to_string(), MathType::Number(consts::PI as Number)),
                ("PI".to_string(), MathType::Number(consts::PI as Number)),
                ("e".to_string(), MathType::Number(consts::E as Number)),
                ("E".to_string(), MathType::Number(consts::E as Number)),
            ]),
            user_functions: HashMap::new(),
            trig_mode: TrigMode::Deg,
            digit_cap: 9,
        }
    }
}

fn execute_line(line: &mut String, environment: &mut Environment) -> Result<(), Box<dyn Error>> {
    get_input(line)?;
    let mut tokens = tokens::generate_tokens(line);
    let command_response = tree_builder::parse_commands(&mut tokens, environment)?;
    match command_response.as_str() {
        "clear" => {}, // todo: clear terminal
        "exit" => process::exit(0),
        "" => {},
        _ => println!("{}", command_response),
    };
    let expression_tree = tree_builder::build_expression_tree(tokens);
    match expression_tree {
        Ok(node) => {
            println!("{}", node);
        },
        Err(e) => println!("{:?}", e),
    };
    Ok(())
}

fn main() {
    let mut executions = 0;
    let mut user_input = String::new();

    let mut environment = Environment::default();

    loop {
        match execute_line(&mut user_input, &mut environment) {
            Ok(()) => {},
            Err(e) => println!("{:?}", e),
        }

        user_input.clear();

        executions += 1;
        if executions > 10 {
            break;
        }
    }
}

fn get_input(buf: &mut String) -> Result<(), io::Error> {
    print!(": ");
    stdout().flush()?;
    stdin().read_line(buf)?;
    Ok(())
}
