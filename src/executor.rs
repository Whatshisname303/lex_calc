use std::collections::HashMap;
use std::error::Error;
use std::f64::consts;
use std::{fmt, iter};

use crate::tree_builder::Node;

#[derive(Debug)]
pub enum ExecutionError {
    UnknownOperator(String),
    UnknownExpression(String),
    UnknownIdentifier(String),
    InvalidOperation(String),
    InvalidVectorContents(String),
    MatrixUnequalRowLengths,
    WrongNumFunctionArgs(usize, usize),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::UnknownOperator(e) => write!(f, "operator '{e}' does not exist"),
            ExecutionError::UnknownExpression(e) => write!(f, "could not parse expression: {e}"),
            ExecutionError::UnknownIdentifier(e) => write!(f, "unknown identifier: {e}"),
            ExecutionError::InvalidOperation(e) => write!(f, "invalid operation: {e}"),
            ExecutionError::InvalidVectorContents(e) => write!(f, "cannot contain '{e}' in vector"),
            ExecutionError::MatrixUnequalRowLengths => write!(f, "matrix row lengths are unequal"),
            ExecutionError::WrongNumFunctionArgs(a, b) => write!(f, "called function requiring {a} params with {b} args"),
        }
    }
}

impl Error for ExecutionError {}

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


#[derive(Debug, Clone)]
pub enum TrigMode {
    Rad,
    Deg,
}

// should probably have a constructor that reads these from config
#[derive(Clone)]
pub struct Environment {
    pub user_vars: HashMap<String, MathType>,
    pub user_functions: HashMap<String, (Node, Node)>, // name to (params, body)
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
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("number + vec".to_string())),
                    MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("number + matrix".to_string())),
                },
                MathType::Vector(lhs) => match rhs {
                    MathType::Number(_) => Err(ExecutionError::InvalidOperation("vec + number".to_string())),
                    MathType::Vector(rhs) => {
                        if lhs.len() != rhs.len() {
                            return Err(ExecutionError::InvalidOperation("adding vectors with different size".to_string()))
                        }
                        Ok(MathType::Vector(lhs.iter().zip(rhs).map(|(l, r)| l + r).collect()))
                    },
                    MathType::Matrix(_rhs) => Err(ExecutionError::InvalidOperation("number + matrix".to_string())),
                },
                MathType::Matrix(lhs) => match rhs {
                    MathType::Number(_) => Err(ExecutionError::InvalidOperation("matrix + number".to_string())),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("matrix + vec".to_string())),
                    MathType::Matrix(rhs) => {
                        let (width, height) = (lhs.len(), lhs.get(0).map(|v| v.len()).unwrap_or(0));
                        let (width2, height2) = (rhs.len(), rhs.get(0).map(|v| v.len()).unwrap_or(0));
                        if width != width2 || height != height2 {
                            return Err(ExecutionError::InvalidOperation("adding matrices with different size".to_string()));
                        }
                        Ok(MathType::Matrix(
                            iter::zip(lhs, rhs).map(|(vec1, vec2)| {
                                iter::zip(vec1, vec2)
                                    .map(|(num1, num2)| num1 + num2)
                                    .collect()
                            }).collect()
                        ))
                    }
                }
            },
            "-" => match self {
                MathType::Number(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Number(lhs - rhs)),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("number - vec".to_string())),
                    MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("number - matrix".to_string())),
                },
                MathType::Vector(lhs) => match rhs {
                    MathType::Number(_) => Err(ExecutionError::InvalidOperation("vec - number".to_string())),
                    MathType::Vector(rhs) => {
                        if lhs.len() != rhs.len() {
                            return Err(ExecutionError::InvalidOperation("subtracting vectors with different size".to_string()))
                        }
                        Ok(MathType::Vector(lhs.iter().zip(rhs).map(|(l, r)| l - r).collect()))
                    },
                    MathType::Matrix(_rhs) => Err(ExecutionError::InvalidOperation("number / matrix".to_string())),
                },
                MathType::Matrix(lhs) => match rhs {
                    MathType::Number(_) => Err(ExecutionError::InvalidOperation("matrix - number".to_string())),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("matrix - vec".to_string())),
                    MathType::Matrix(rhs) => {
                        let (width, height) = (lhs.len(), lhs.get(0).map(|v| v.len()).unwrap_or(0));
                        let (width2, height2) = (rhs.len(), rhs.get(0).map(|v| v.len()).unwrap_or(0));
                        if width != width2 || height != height2 {
                            return Err(ExecutionError::InvalidOperation("subtracting matrices with different size".to_string()));
                        }
                        Ok(MathType::Matrix(
                            iter::zip(lhs, rhs).map(|(vec1, vec2)| {
                                iter::zip(vec1, vec2)
                                    .map(|(num1, num2)| num1 - num2)
                                    .collect()
                            }).collect()
                        ))
                    },
                },
            },
            "*" => match self {
                MathType::Number(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Number(lhs * rhs)),
                    MathType::Vector(rhs) => Ok(MathType::Vector(rhs.iter().map(|v| v * lhs).collect())),
                    MathType::Matrix(rhs) => Ok(MathType::Matrix(rhs.iter().map(|vec| vec.iter().map(|num| num * lhs).collect()).collect())),
                },
                MathType::Vector(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Vector(lhs.iter().map(|v| v * rhs).collect())),
                    MathType::Vector(_) => Err(ExecutionError::InvalidOperation("vec * vec".to_string())),
                    MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("vector * matrix".to_string())),
                },
                MathType::Matrix(lhs) => match rhs {
                    MathType::Number(rhs) => Ok(MathType::Matrix(lhs.iter().map(|vec| vec.iter().map(|num| num * rhs).collect()).collect())),
                    MathType::Vector(rhs) => {
                        let (width1, height1) = (lhs.len(), lhs.get(0).map(|v| v.len()).unwrap_or(0));
                        let height2 = rhs.len();
                        if width1 != height2 {
                            return Err(ExecutionError::InvalidOperation("matrix width does not match vector height".to_string()));
                        }
                        let mut vector = Vec::new();
                        let mut row = 0;
                        while row < height1 {
                            let mut col = 0;
                            let mut dotsum = 0.0;
                            while col < width1 {
                                dotsum += lhs[col][row] * rhs[col];
                                col += 1;
                            }
                            vector.push(dotsum);
                            row += 1;
                        }
                        Ok(MathType::Vector(vector))
                    },
                    MathType::Matrix(rhs) => {
                        let (width1, height1) = (lhs.len(), lhs.get(0).map(|v| v.len()).unwrap_or(0));
                        let (width2, height2) = (rhs.len(), rhs.get(0).map(|v| v.len()).unwrap_or(0));
                        if width1 != height2 {
                            return Err(ExecutionError::InvalidOperation("matrix1 width does not match matrix2 height".to_string()));
                        }
                        let mut matrix: Vec<Vec<f64>> = Vec::new();
                        let mut row1 = 0;
                        while row1 < height1 {
                            let mut i = 0;
                            while i < width2 {
                                let mut dotsum = 0.0;
                                let mut col1 = 0;
                                while col1 < width1 {
                                    dotsum += lhs[col1][row1] * rhs[i][col1];
                                    col1 += 1;
                                }
                                match matrix.get_mut(i) {
                                    Some(vector) => vector.push(dotsum),
                                    None => matrix.push(vec![dotsum]),
                                }
                                i += 1;
                            }
                            row1 += 1;
                        }
                        Ok(MathType::Matrix(matrix))
                    },
                },
            },
            "/" => match self {
                    MathType::Number(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Number(lhs / rhs)),
                        MathType::Vector(_) => Err(ExecutionError::InvalidOperation("number / vec".to_string())),
                        MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("number / matrix".to_string())),
                    },
                    MathType::Vector(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Vector(lhs.iter().map(|v| v / rhs).collect())),
                        MathType::Vector(_) => Err(ExecutionError::InvalidOperation("vec / vec".to_string())),
                        MathType::Matrix(_) => Err(ExecutionError::InvalidOperation("vec / matrix".to_string())),
                    },
                    MathType::Matrix(lhs) => match rhs {
                        MathType::Number(rhs) => Ok(MathType::Matrix(lhs.iter().map(|vec| vec.iter().map(|num| num * rhs).collect()).collect())),
                        MathType::Vector(_rhs) => todo!(),
                        MathType::Matrix(_rhs) => todo!(),
                    },
            },
            "//" => {
                todo!();
            },
            _ => Err(ExecutionError::UnknownOperator(operator.to_string())),
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

fn handle_matrix_creation(nodes: &Vec<Node>, environment: &mut Environment) -> Result<MathType, ExecutionError> {
    let mut vectors: Vec<Vec<Number>> = Vec::new();
    let mut col = 1;

    for node in nodes.iter().skip(1) {
        if node.is_str(",") || node.is_str("]") {
            continue;
        } else if node.is_str(";") {
            col = 0;
        } else {
            let element = execute_expression_tree(node, environment)?;
            match element {
                MathType::Number(num) => match vectors.get_mut(col) {
                    Some(vector) => vector.push(num),
                    None => vectors.push(vec![num]),
                },
                _ => return Err(ExecutionError::InvalidVectorContents(element.to_string())),
            }
            col += 1;
        }
    }

    let height = vectors.get(0).map(|v| v.len()).unwrap_or(0);
    if vectors.iter().any(|vector| vector.len() != height) {
        return Err(ExecutionError::MatrixUnequalRowLengths);
    }

    if vectors.len() == 0 {
        return Ok(MathType::Vector(Vec::new()));
    } else if vectors.len() == 1 {
        return Ok(MathType::Vector(vectors.remove(0)));
    } else {
        return Ok(MathType::Matrix(vectors));
    }
}

fn handle_user_function_call(body: &Node, params: &Node, args: &Node, environment: &Environment) -> Result<MathType, ExecutionError> {
    let mut function_environment = environment.clone();

    let params: Vec<String> = match params {
        Node::Tkn(token) => vec![token.clone()],
        Node::Exp(subnodes) => subnodes.iter()
            .filter(|node| !node.is_str(","))
            .filter_map(|node| node.as_identifier())
            .map(|node| node.clone())
            .collect()
    };

    let args = match args {
        Node::Tkn(_) => vec![execute_expression_tree(args, &mut function_environment)],
        Node::Exp(subnodes) => subnodes
            .split(|e| e.is_str(","))
            .map(|nodes| execute_expression_tree(&Node::Exp(nodes.to_vec()), &mut function_environment))
            .collect()
    };

    if params.len() != args.len() {
        return Err(ExecutionError::WrongNumFunctionArgs(params.len(), args.len()));
    }

    for (param, arg) in iter::zip(params, args) {
        function_environment.user_vars.insert(param, arg?);
    }

    execute_expression_tree(body, &mut function_environment)
}

fn handle_builtin_function_call(function_name: &String, arguments: &Node) -> Result<MathType, ExecutionError> {
    todo!() // todo: implement
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
            if let Some(first) = subnodes.get(0) {
                if first.is_str("[") {
                    return handle_matrix_creation(subnodes, environment);
                }
            }

            match subnodes.len() {
                0 => Ok(MathType::Number(0.0)),
                1 => execute_expression_tree(&subnodes[0], environment),
                2 => {
                    let left_node = &subnodes[0];
                    let right_node = &subnodes[1];

                    if left_node.is_unary_operator() { // only expecting '-' currently
                        execute_expression_tree(right_node, environment)?.operate("*", MathType::Number(-1.0))

                    } else if let Node::Tkn(token) = left_node { // expecting function call
                        if let Some((params, body)) = environment.user_functions.get(token) {
                            handle_user_function_call(body, params, right_node, environment) // right node will be argument expression (...)

                        } else if let Some(func) = environment.user_functions.get(token) {
                            // todo: swap out the condition to match builtin functions; implement
                            todo!()
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
