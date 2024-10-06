use std::io::{self, stdin, stdout, Write};
use std::collections::HashMap;

mod tokens;
mod tree_builder;

struct Environment {
    // user_vars: HashMap<String, MathType>,
    user_functions: HashMap<String, i32>, // need to define actual function type at some point
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            // user_vars: HashMap::new(),
            user_functions: HashMap::new(),
        }
    }
}

fn main() {
    let mut executions = 0;
    let mut user_input = String::new();

    let mut environment = Environment::default();

    loop {
        match get_input(&mut user_input) {
            Ok(()) => {
                let tokens = tokens::generate_tokens(&user_input);
                let expression_tree = tree_builder::build_expression_tree(tokens);
                match expression_tree {
                    Ok(node) => println!("{:?}", node),
                    Err(e) => println!("{:?}", e),
                };
            },
            Err(e) => {
                println!("Couldn't read input: {e}");
            },
        };

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
