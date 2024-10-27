use std::iter;

use crate::executor::{MathType, ExecutionError};

impl MathType {
    pub fn operate(&self, operator: &str, rhs: MathType) -> Result<MathType, ExecutionError> {
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
