use std::io::{self, stdin, stdout, Write};
use tokens::Token;

mod tokens;

fn main() {
    let mut executions = 0;
    let mut user_input = String::new();
    loop {
        match get_input(&mut user_input) {
            Ok(()) => {
                match tokens::generate_tokens(&user_input) {
                    Ok(tokens) => {
                        for token in tokens {
                            match token {
                                Token::Operator(operator) => {
                                    print!("op:'{:?}' ", operator);
                                },
                                Token::Text(text) => {
                                    print!("text:'{text}' ");
                                },
                            };
                        }
                        println!("-- Parsed token output")
                    },
                    Err(e) => {
                        println!("Fucked up token generation: {:?}", e);
                    }
                }
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
