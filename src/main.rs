use std::io::{self, stdin, stdout, Write};

fn main() {
    let mut executions = 0;
    let mut user_input = String::new();
    loop {
        match get_input(&mut user_input) {
            Ok(()) => {
                match generate_tokens(&user_input) {
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

#[derive(Debug)]
enum Token {
    Operator(Operator),
    Text(String),
}

#[derive(Clone, Debug)]
enum Operator {
    Assignment,
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Addr,
    Gt,
    Ge,
    Lt,
    Le,
    Dot,
    Comma,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    CurlyOpen,
    CurlyClose,
    Arrow,
    Wacky,
}

const TOKEN_MAP: [(&str, Operator); 21] = [
    ("=", Operator::Assignment),
    ("+", Operator::Plus),
    ("-", Operator::Minus),
    ("*", Operator::Mult),
    ("/", Operator::Div),
    ("^", Operator::Pow),
    ("&", Operator::Addr),
    (">", Operator::Gt),
    (">=", Operator::Ge),
    ("<", Operator::Lt),
    ("<=", Operator::Le),
    (".", Operator::Dot),
    (",", Operator::Comma),
    ("(", Operator::ParenOpen),
    (")", Operator::ParenClose),
    ("[", Operator::BracketOpen),
    ("]", Operator::BracketClose),
    ("{", Operator::CurlyOpen),
    ("}", Operator::CurlyClose),
    ("=>", Operator::Arrow),
    ("$&", Operator::Wacky),
];

#[allow(dead_code)]
#[derive(Debug)]
enum TokenizeError {
    IncompleteOperator(String),
}

fn generate_tokens(text: &String) -> Result<Vec<Token>, TokenizeError> {
    let mut token_sequence: Vec<Token> = Vec::new();
    let mut current_text = String::new();

    let mut is_text = true;

    let chars = text.chars().skip_while(|c| c.is_whitespace());

    for c in chars {
        if is_text {
            let possible_token = TOKEN_MAP.iter().find(|entry| entry.0.starts_with(c));
            if possible_token.is_some() {
                if current_text != "" {
                    token_sequence.push(Token::Text(current_text)); // break text with operator
                    current_text = String::new();
                }
                is_text = false;
            }
            if c.is_whitespace() {
                if current_text != "" {
                    token_sequence.push(Token::Text(current_text)); // break text with whitespace
                    current_text = String::new();
                }
            } else {
                current_text.push(c);
            }
        } else {
            let last_match = TOKEN_MAP.iter().find(|entry| entry.0 == current_text);
            current_text.push(c);
            let new_match = TOKEN_MAP.iter().find(|entry| entry.0 == current_text);
            if new_match.is_none() {
                if let Some(token) = last_match {
                    token_sequence.push(Token::Operator(token.1.clone())); // operator finished
                    current_text.clear();
                    if !c.is_whitespace() {
                        current_text.push(c);
                    }
                    is_text = true;
                } else {
                    return Err(TokenizeError::IncompleteOperator(current_text));
                }
            }
        }
    }

    if current_text != "" {
        token_sequence.push(Token::Text(current_text));
    }

    Ok(token_sequence)
}

fn get_input(buf: &mut String) -> Result<(), io::Error> {
    print!(": ");
    stdout().flush()?;
    stdin().read_line(buf)?;
    Ok(())
}
