use std::error::Error;
use std::io::{self, stdin, stdout, Write};
use std::process;

mod tokens;
mod tree_builder;
mod executor;

fn execute_line(line: &mut String, environment: &mut executor::Environment) -> Result<(), Box<dyn Error>> {
    get_input(line)?;

    let mut tokens = tokens::generate_tokens(line);
    let command_response = tree_builder::parse_commands(&mut tokens, environment)?;

    let processed = match command_response.as_str() {
        "clear" => true, // todo: clear terminal
        "exit" => process::exit(0),
        "new function" => {
            println!("new function");
            true
        },
        "" => false,
        _ => {
            println!("{}", command_response);
            true
        },
    };
    if processed {
        return Ok(());
    }

    let expression_tree = tree_builder::build_expression_tree(tokens);
    match expression_tree {
        Ok(nodes) => {
            // println!("{}", nodes);
            match executor::execute_expression_tree(&nodes, environment) {
                Ok(value) => {
                    println!("{}\n", value);
                    environment.user_vars.insert("ans".to_string(), value);
                },
                Err(e) => println!("{}", e),
            }
        },
        Err(e) => println!("{:?}", e),
    };
    Ok(())
}

fn main() {
    let mut user_input = String::new();

    let mut environment = executor::Environment::default();

    loop {
        match execute_line(&mut user_input, &mut environment) {
            Ok(()) => {},
            Err(e) => println!("{:?}", e),
        }

        user_input.clear();
    }
}

fn get_input(buf: &mut String) -> Result<(), io::Error> {
    print!(": ");
    stdout().flush()?;
    stdin().read_line(buf)?;
    Ok(())
}
