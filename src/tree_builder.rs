use std::error::Error;
use std::fmt;

use crate::{
    tokens::Token,
    executor::{Environment, TrigMode},
};

const UNARY_OPERATORS: &'static [&'static str] = &["-", "&", "!"];
const BINARY_OPERATOR_PRIORITY: &'static [&'static [&'static str]] = &[
    &["^"],
    &["*", "/", "//"],
    &["+", "-"],
    &["=>", "="],
];

#[derive(Debug)]
pub enum ExpressionBuildError {
    HangingBrace(String),
    InvalidMode(String),
}

impl fmt::Display for ExpressionBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionBuildError::HangingBrace(e) => write!(f, "missing closing {}", e),
            ExpressionBuildError::InvalidMode(e) => write!(f, "mode update error: {e}"),
        }
    }
}

impl Error for ExpressionBuildError {}

#[derive(Debug, Clone)]
pub enum Node {
    Tkn(Token),
    Exp(Vec<Node>),
}

impl Node {
    pub fn is_operator(&self) -> bool {
        self.is_unary_operator() || self.is_binary_operator()
    }
    pub fn is_unary_operator(&self) -> bool {
        match self {
            Node::Exp(_) => false,
            Node::Tkn(token) => {
                UNARY_OPERATORS.contains(&token.as_str())
            }
        }
    }
    pub fn is_binary_operator(&self) -> bool {
        match self {
            Node::Exp(_) => false,
            Node::Tkn(token) => BINARY_OPERATOR_PRIORITY.iter().any(|prio| prio.contains(&token.as_str())),
        }
    }
    pub fn is_str(&self, s: &str) -> bool {
        match self {
            Node::Exp(_) => false,
            Node::Tkn(token) => token == s,
        }
    }
    // other is_..._operator functions should also return option rather than bool, but idc rn
    pub fn as_assignment_operator(&self) -> Option<&String> {
        match self {
            Node::Exp(_) => None,
            Node::Tkn(token) => match token.as_str() {
                "=" | "=>" => Some(token),
                _ => None,
            },
        }
    }
    pub fn as_identifier(&self) -> Option<&String> {
        match self {
            Node::Exp(_) => None,
            Node::Tkn(token) => Some(token),
        }
    }
    pub fn unrolled(&self) -> &Node {
        match self {
            Node::Exp(subnodes) => match subnodes.len() {
                1 => subnodes[0].unrolled(),
                _ => self,
            },
            Node::Tkn(_) => self
        }
    }
    pub fn flat_string(&self) -> String {
        let mut res = String::new();
        match self {
            Node::Tkn(token) => res += &format!("'{}', ", token),
            Node::Exp(subnodes) => subnodes.iter().for_each(|node| res += &node.flat_string()),
        }
        res
    }
    fn pretty_string(&self, depth: usize) -> String {
        let mut res = String::new();

        for _ in 0..depth {
            res.push_str("\t");
        }

        match self {
            Node::Tkn(token) => {
                res.push_str(format!("'{}',\n", token).as_str());
            },
            Node::Exp(subnodes) => {
                res.push_str("[\n");
                for node in subnodes {
                    res.push_str(node.pretty_string(depth + 1).as_str());
                }
                for _ in 0..depth {
                    res.push_str("\t");
                }
                res.push_str("]\n");
            },
        }
        res
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_string(0))
    }
}

// --------------------------------------------------------------------------------------------------------------------

fn parse_square_braces(nodes: &mut Vec<Node>) -> Result<(), ExpressionBuildError> {
    let mut i = 0;
    while i < nodes.len() {
        if nodes[i].is_str("[") {
            let mut j = i;
            loop {
                j += 1;
                if j >= nodes.len() {
                    return Err(ExpressionBuildError::HangingBrace("]".to_string()));
                }
                if nodes[j].is_str("]") {
                    break;
                }
            };
            let subnodes: Vec<Node> = nodes.drain(i..=j).collect();
            nodes.insert(i, Node::Exp(subnodes));
        };
        i += 1;
    }
    Ok(())
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

fn fill_missing_ans(nodes: &mut Vec<Node>) {
    match nodes.get(0) {
        Some(node) => {
            if node.is_binary_operator() {
                nodes.insert(0, Node::Tkn(Token::from("ans")));
            }
        },
        None => {}
    };
}

fn parse_functions(nodes: &mut Vec<Node>) {
    let mut i = 1;
    while (i as isize) < (nodes.len() as isize) - 1 {
        let current = &nodes[i].unrolled();
        let next = &nodes[i + 1].unrolled();
        if !current.is_operator() && !next.is_operator() {
            println!("Found function to eval");
            let mut function_nodes: Vec<Node> = nodes.drain(i..=i+1).collect();
            parse_functions(&mut function_nodes);
            nodes.insert(i, Node::Exp(function_nodes));
        }
        i += 1;
    }
}

fn parse_unary(nodes: &mut Vec<Node>) {
    let mut i = 0;
    while (i as isize) < (nodes.len() as isize) - 1 {
        match nodes[i] {
            Node::Tkn(_) => {
                if nodes[i].is_unary_operator() && (i == 0 || nodes[i - 1].is_operator()) {
                    let unary_nodes: Vec<Node> = nodes.drain(i..=(i + 1)).collect();
                    nodes.insert(i, Node::Exp(unary_nodes));
                }
            },
            Node::Exp(ref mut subnodes) => {
                parse_unary(subnodes);
            },
        };
        i += 1;
    }
}

fn parse_binary(nodes: &mut Vec<Node>) {
    for ops in BINARY_OPERATOR_PRIORITY {
        let mut i = 1;
        while (i as isize) < (nodes.len() as isize) - 1 {
            match nodes[i] {
                Node::Exp(ref mut subnodes) => {
                    parse_binary(subnodes);
                },
                Node::Tkn(ref token) => {
                    if ops.contains(&token.as_str()) {
                        let binary_nodes: Vec<Node> = nodes.drain(i-1..=i+1).collect();
                        nodes.insert(i - 1, Node::Exp(binary_nodes));
                    }
                }
            };
            i += 1;
        }
    }
}

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
                    token_sequence.remove(0);
                    let vars: String = environment.user_vars
                        .iter()
                        .map(|(name, value)| format!("{:?} = {:?}\n", name, value))
                        .collect();
                    let functions: String = environment.user_functions
                        .iter()
                        .map(|(name, nodes)| format!("function {} = {}\n", name, Node::Exp(nodes.clone()).flat_string()))
                        .collect();
                    Ok(format!("display digits: {}\ntrig mode: {:?}\nvars:\n{}\nfunctions:\n{}", environment.digit_cap, environment.trig_mode, vars, functions))
                }
            },
            "clearvars" => {
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
    let mut nodes: Vec<Node> = token_sequence.iter().map(|token| Node::Tkn(token.clone())).collect();
    parse_square_braces(&mut nodes)?;
    nodes = parse_tree_braces(nodes)?;
    fill_missing_ans(&mut nodes);
    println!("About to pass functions with {}", Node::Exp(nodes.clone()));
    parse_functions(&mut nodes);
    println!("With functions parsed: {}", Node::Exp(nodes.clone()));
    parse_unary(&mut nodes);
    parse_binary(&mut nodes);
    Ok(Node::Exp(nodes))
}
