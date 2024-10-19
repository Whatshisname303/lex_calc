use std::collections::HashMap;
use std::f64::consts;
use std::fmt;

use crate::tree_builder::Node;

#[derive(Debug)]
pub enum ExecutionError {
    UnknownOperator(String),
    UnknownExpression(String),
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

impl fmt::Display for MathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathType::Number(number) => write!(f, "{}", number),
            MathType::Vector(vector) => {
                write!(f, "[")?;
                if let Some(first) = vector.get(0) {
                    write!(f, "{}", first)?
                }
                for number in vector.iter().skip(1) {
                    write!(f, ", {}", number)?;
                }
                write!(f, "]")
            },
            MathType::Matrix(matrix) => {
                let width = matrix.len();
                let height = matrix.get(0).map(|r| r.len()).unwrap_or(0);
                write!(f, "[\n")?;
                let mut row = 0;
                while row < height {
                    write!(f, "\t")?;
                    let mut col = 0;
                    while col < width {
                        write!(f, "{}, ", matrix[col][row])?;
                        col += 1;
                    }
                    write!(f, "\n")?;
                    row += 1;
                }
                write!(f, "]\n")
            },
        }
    }
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
            "^" => match self {
                MathType::Number(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Number(lhs.powf(rhs))),
                    MathType::Vector(_rhs) => todo!(), // there might be some weird way to do this
                    MathType::Matrix(_rhs) => todo!(),
                },
                MathType::Vector(_lhs) => todo!(),
                MathType::Matrix(_lhs) => todo!(),
            },
            "+" => match self {
                MathType::Number(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Number(lhs + rhs)),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("cannot do number + vec".to_string())),
                    MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("cannot do number + matrix".to_string())),
                },
                MathType::Vector(lhs) => match rhs {
                    MathType::Number(_) => Err(ExecutionError::InvalidOperation("cannot do vec + number".to_string())),
                    MathType::Vector(rhs) => {
                        if lhs.len() != rhs.len() {
                            return Err(ExecutionError::InvalidOperation("cannot add vectors with different length".to_string()))
                        }
                        Ok(MathType::Vector(lhs.iter().zip(rhs).map(|(l, r)| l + r).collect()))
                    },
                    MathType::Matrix(_rhs) => Err(ExecutionError::InvalidOperation("cannot do number + matrix".to_string())),
                },
                MathType::Matrix(_lhs) => {
                    todo!()
                }
            },
            "-" => match self {
                MathType::Number(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Number(lhs - rhs)),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("cannot do number - vec".to_string())),
                    MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("cannot do number - matrix".to_string())),
                },
                MathType::Vector(lhs) => match rhs {
                    MathType::Number(_) => Err(ExecutionError::InvalidOperation("cannot do vec - number".to_string())),
                    MathType::Vector(rhs) => {
                        if lhs.len() != rhs.len() {
                            return Err(ExecutionError::InvalidOperation("cannot subtract vectors with different length".to_string()))
                        }
                        Ok(MathType::Vector(lhs.iter().zip(rhs).map(|(l, r)| l - r).collect()))
                    },
                    MathType::Matrix(_rhs) => Err(ExecutionError::InvalidOperation("cannot do number / matrix".to_string())),
                },
                MathType::Matrix(_lhs) => match rhs {
                    MathType::Number(_rhs) => todo!(),
                    MathType::Vector(_rhs) => todo!(),
                    MathType::Matrix(_rhs) => todo!(),
                },
            },
            "*" => match self {
                MathType::Number(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Number(lhs * rhs)),
                    MathType::Vector(rhs) => Ok(MathType::Vector(rhs.iter().map(|v| v * lhs).collect())),
                    MathType::Matrix(_rhs) => todo!(),
                },
                MathType::Vector(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Vector(lhs.iter().map(|v| v * rhs).collect())),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("cannot do vec * vec".to_string())),
                    MathType::Matrix(_rhs) => todo!(),
                },
                MathType::Matrix(_lhs) => match rhs {
                    MathType::Number(_rhs) => todo!(),
                    MathType::Vector(_rhs) => todo!(),
                    MathType::Matrix(_rhs) => todo!(),
                },
            },
            "/" => match self {
                    MathType::Number(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Number(lhs / rhs)),
                        MathType::Vector(_) => Err(ExecutionError::InvalidOperation("cannot do number / vec".to_string())),
                        MathType::Matrix(_rhs) => todo!(),
                    },
                    MathType::Vector(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Vector(lhs.iter().map(|v| v / rhs).collect())),
                        MathType::Vector(_) => Err(ExecutionError::InvalidOperation("cannot do vec / vec".to_string())),
                        MathType::Matrix(_rhs) => todo!(),
                    },
                    MathType::Matrix(_lhs) => match rhs {
                        MathType::Number(_rhs) => todo!(),
                        MathType::Vector(_rhs) => todo!(),
                        MathType::Matrix(_rhs) => todo!(),
                    },
            },
            "//" => {
                Ok(MathType::Number(1.0)) // todo
            },
            _ => {
                Err(ExecutionError::UnknownOperator(operator.to_string()))
            }
        }
    }
}

fn handle_assignment(lhs: &Node, rhs: &Node, operator: &str, environment: &mut Environment) -> Result<MathType, ExecutionError> {
    let (identifier, value) = match operator {
        "=" => (lhs, rhs),
        "=>" => (rhs, lhs),
        _ => panic!("should be an assignment operator"),
    };
    let identifier = identifier.as_identifier().ok_or(ExecutionError::InvalidOperation("invalid variable name".to_string()))?;
    let value = execute_expression_tree(value, environment)?;
    environment.user_vars.insert(identifier.to_string(), value.clone());
    Ok(value)
}

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
                0 => Ok(MathType::Number(0.0)),
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
                        Err(ExecutionError::UnknownExpression(format!("left: {}; right: {};", left_node.flat_string(), right_node.flat_string())))
                    }
                },
                3 => {
                    let operator = &subnodes[1];
                    if let Some(assignment_operator) = operator.as_assignment_operator() {
                        handle_assignment(&subnodes[0], &subnodes[2], assignment_operator, environment)
                    } else if let Node::Tkn(op) = operator {
                        let lhs = execute_expression_tree(&subnodes[0], environment)?;
                        let rhs = execute_expression_tree(&subnodes[2], environment)?;
                        lhs.operate(&op, rhs)
                    } else {
                        Err(ExecutionError::UnknownExpression(format!("lhs: {}; op: {}; rhs: {}", subnodes[0].flat_string(), operator.flat_string(), subnodes[1].flat_string())))
                    }
                },
                _ => {
                    Err(ExecutionError::UnknownExpression(String::from_iter(subnodes.iter().map(|node| node.flat_string()))))
                },
            }
        }
    }
}
