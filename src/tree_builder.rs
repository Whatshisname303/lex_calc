use std::error::Error;
use std::fmt;

use crate::{tokens::Token, Environment, TrigMode};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ExpressionBuildError {
    NoClosingBrace,
    InvalidMode(String),
}

impl fmt::Display for ExpressionBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionBuildError::NoClosingBrace => write!(f, "missing closing ')'"),
            ExpressionBuildError::InvalidMode(e) => write!(f, "mode update error: {e}"),
        }
    }
}

impl Error for ExpressionBuildError {}

#[derive(Debug)]
pub enum Node {
    Tkn(Token),
    Exp(Vec<Node>),
}

fn parse_tree_braces(mut nodes: Vec<Node>) -> Result<Vec<Node>, ExpressionBuildError> {
    let mut i = 0;
    while i < nodes.len() {
        if let Node::Tkn(token) = &nodes[i] {
            match token.as_str() {
                ")" => {
                    let mut remaining_nodes = nodes.split_off(i + 1);
                    nodes.pop(); // remove ")"

                    let expression = Node::Exp(nodes);

                    let mut new_nodes = Vec::with_capacity(remaining_nodes.len() + 1);

                    new_nodes.push(expression);
                    new_nodes.append(&mut remaining_nodes);
                    return Ok(new_nodes);
                },
                "(" => {
                    let contained_nodes = nodes.split_off(i + 1);
                    nodes.pop(); // remove "("

                    let mut parsed_nodes = parse_tree_braces(contained_nodes)?;
                    nodes.append(&mut parsed_nodes);
                    return Ok(nodes);
                },
                _ => {},
            }
        }
        i += 1;
    }
    Ok(nodes)
}

// todo: remove mode update tokens from node list
// todo: allow temporary mode updates if tokens continue past mode update
pub fn parse_commands(token_sequence: &mut Vec<Token>, environment: &mut Environment) -> Result<String, ExpressionBuildError> {
    match token_sequence.get(0) {
        Some(token) => match token.as_str() {
            "clear" => {
                token_sequence.clear();
                Ok(String::from("clear"))
            },
            "mode" => match token_sequence.get(1) {
                Some(token) => match token.as_str() {
                    "rad" => {
                        environment.trig_mode = TrigMode::Rad;
                        token_sequence.drain(..2);
                        Ok(String::from("set mode to radians"))
                    },
                    "deg" => {
                        environment.trig_mode = TrigMode::Deg;
                        token_sequence.drain(..2);
                        Ok(String::from("set mode to degrees"))
                    },
                    "digits" => match token_sequence.get(2) {
                        Some(token) => match token.parse::<u8>() {
                            Ok(digit) => {
                                environment.digit_cap = digit;
                                token_sequence.drain(..3);
                                Ok(format!("set display digits to {digit} digits"))
                            },
                            Err(_) => Err(ExpressionBuildError::InvalidMode(format!("could not parse digit, got '{}'", token)))
                        },
                        None => Err(ExpressionBuildError::InvalidMode(format!("must provide number of digits to display")))
                    }
                    _ => Err(ExpressionBuildError::InvalidMode(format!("no option to change mode '{}'", token)))
                }
                None => {
                    let vars: String = environment.user_vars
                        .iter()
                        .map(|(name, value)| format!("{:?} = {:?}\n", name, value))
                        .collect(); // todo: chain user functions if they don't end up merged into vars
                    Ok(format!("display digits: {}\ntrig mode: {:?}\nvars:\n{}", environment.digit_cap, environment.trig_mode, vars))
                }
            },
            "newvars" => {
                let default_env = Environment::default();
                environment.user_vars = default_env.user_vars;
                environment.user_functions = default_env.user_functions;
                token_sequence.clear();
                Ok(String::from("cleared vars"))
            },
            "quit" | "exit" | "q" => {
                Ok(String::from("exit"))
            }
            _ => Ok(String::new()),
        },
        None => Ok(String::new()),
    }
}

pub fn build_expression_tree(token_sequence: Vec<Token>) -> Result<Node, ExpressionBuildError> {
    let nodes: Vec<Node> = token_sequence.iter().map(|token| Node::Tkn(token.clone())).collect();
    let nodes = parse_tree_braces(nodes)?;
    // parse functions
    // parse operators
    // execute
    // TODO: mutate root expression going through full order of operations
    Ok(Node::Exp(nodes))
}
