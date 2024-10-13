use std::collections::HashMap;
use std::f64::consts;

use crate::tree_builder::Node;

#[derive(Debug)]
pub enum ExecutionError {
    UnexpectedExpressionLength(usize),
    UnknownOperator(String),
    UnknownExpression(usize),
    UnknownIdentifier(String),
    InvalidOperation(String),
}

pub type Number = f64;

#[derive(Debug, Clone)]
pub enum MathType {
    Number(Number),
    Vector(Vec<Number>),
    Matrix(Vec<Vec<Number>>),
}


#[derive(Debug)]
pub enum TrigMode {
    Rad,
    Deg,
}

// should probably have a constructor that reads these from config
pub struct Environment {
    pub user_vars: HashMap<String, MathType>,
    pub user_functions: HashMap<String, i32>, // need to define actual function type at some point
    pub trig_mode: TrigMode,
    pub digit_cap: u8,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            user_vars: HashMap::from([
                ("pi".to_string(), MathType::Number(consts::PI as Number)),
                ("PI".to_string(), MathType::Number(consts::PI as Number)),
                ("e".to_string(), MathType::Number(consts::E as Number)),
                ("E".to_string(), MathType::Number(consts::E as Number)),
                ("ans".to_string(), MathType::Number(0 as Number)),
            ]),
            user_functions: HashMap::new(),
            trig_mode: TrigMode::Deg,
            digit_cap: 9,
        }
    }
}

impl MathType {
    fn operate(&self, operator: &str, rhs: MathType) -> Result<MathType, ExecutionError> {
        match operator {
            "^" => {
                Ok(MathType::Number(1.0)) // todo
            },
            "+" => {
                match self {
                    MathType::Number(lhs) => {
                        if let MathType::Number(rhs) = rhs {
                            Ok(MathType::Number(lhs + rhs))
                        } else {
                            Err(ExecutionError::InvalidOperation(format!("cannot add {:?} to {:?}", rhs, lhs)))
                        }
                    },
                    MathType::Vector(_lhs) => {
                        todo!()
                    }
                    MathType::Matrix(_lhs) => {
                        todo!()
                    }
                }
            },
            "-" => {
                Ok(MathType::Number(1.0)) // todo
            },
            "*" => {
                match self {
                    MathType::Number(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Number(lhs * rhs)),
                        MathType::Vector(rhs) => Ok(MathType::Vector(rhs.iter().map(|v| v * lhs).collect())),
                        MathType::Matrix(_rhs) => todo!(),
                    },
                    MathType::Vector(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Vector(lhs.iter().map(|v| v * rhs).collect())),
                        MathType::Vector(_) => Err(ExecutionError::InvalidOperation("attempted invalid vec * number".to_string())),
                        MathType::Matrix(_rhs) => todo!(),
                    },
                    MathType::Matrix(_lhs) => match rhs {
                        MathType::Number(_rhs) => todo!(),
                        MathType::Vector(_rhs) => todo!(),
                        MathType::Matrix(_rhs) => todo!(),
                    }
                }
            },
            "/" => {
                Ok(MathType::Number(1.0)) // todo
            },
            "//" => {
                Ok(MathType::Number(1.0)) // todo
            },
            "=" => {
                Ok(MathType::Number(1.0)) // todo
            },
            "=>" => {
                Ok(MathType::Number(1.0)) // todo
            },
            _ => {
                Err(ExecutionError::UnknownOperator(operator.to_string()))
            }
        }
    }
}


// todo: fix the error handling, this is complete garbage
pub fn execute_expression_tree(root_node: &Node, environment: &mut Environment) -> Result<MathType, ExecutionError> {
    match root_node {
        Node::Tkn(token) => {
            if let Ok(number) = token.parse::<Number>() {
                Ok(MathType::Number(number))
            } else if let Some(value) = environment.user_vars.get(token) {
                Ok(value.clone())
            } else {
                Err(ExecutionError::UnknownIdentifier(token.clone()))
            }
        },
        Node::Exp(subnodes) => {
            match subnodes.len() {
                1 => {
                    execute_expression_tree(&subnodes[0], environment)
                },
                2 => {
                    let left_node = &subnodes[0];
                    let right_node = &subnodes[1];

                    if left_node.is_unary_operator() { // only expecting '-' currently
                        execute_expression_tree(right_node, environment)?.operate("*", MathType::Number(-1.0))
                    } else if let Node::Tkn(token) = left_node { // expecting function call
                        if let Some(_func) = environment.user_functions.get(token) {
                            // todo: process function call
                            Ok(MathType::Number(1 as Number))
                        } else {
                            Err(ExecutionError::UnknownIdentifier(token.clone()))
                        }
                    } else {
                        Err(ExecutionError::UnknownExpression(2))
                    }
                },
                3 => {
                    let lhs =  execute_expression_tree(&subnodes[0], environment)?;
                    let operator = &subnodes[1];
                    let rhs = execute_expression_tree(&subnodes[2], environment)?;
                    if let Node::Tkn(op) = operator {
                        lhs.operate(&op, rhs)
                    } else {
                        Err(ExecutionError::UnknownExpression(3))
                    }
                },
                _ => {
                    Err(ExecutionError::UnexpectedExpressionLength(subnodes.len()))
                },
            }
        }
    }
}
