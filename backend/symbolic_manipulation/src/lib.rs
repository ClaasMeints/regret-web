#![allow(dead_code)]
#![allow(unused_macros)]
use core::fmt;
use std::collections::HashMap;

#[derive(Clone)]
enum Number {
    Complex(f64, f64),
}

#[derive(Clone, PartialEq)]
struct Symbol {
    name: String,
}

impl Symbol {
    fn new(name: String) -> Symbol {
        Symbol { name }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq)]
enum Operator {
    Negation,
    Factorial,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Logarithm,
    Root,
}

enum Size {
    Finite(usize),
    Infinite,
}

impl Operator {
    fn arity(&self) -> Size {
        match self {
            Operator::Negation | Operator::Factorial => Size::Finite(1),
            Operator::Addition
            | Operator::Subtraction
            | Operator::Multiplication
            | Operator::Division => Size::Infinite,
            Operator::Exponentiation | Operator::Logarithm | Operator::Root => Size::Finite(2),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Negation => write!(f, "-"),
            Operator::Factorial => write!(f, "!"),
            Operator::Addition => write!(f, "+"),
            Operator::Subtraction => write!(f, "-"), // unexsitent
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Exponentiation => write!(f, "^"),
            Operator::Logarithm => write!(f, "log"),
            Operator::Root => write!(f, "root"),
        }
    }
}

#[derive(Clone)]
struct OperatorExpression {
    operator: Operator,
    operands: Vec<Expression>,
}

#[derive(Clone)]
enum Expression {
    Constant(i128),
    Symbol(Symbol),
    OperatorExpression(OperatorExpression),
}

impl Expression {
    fn new(
        operator: Operator,
        operands: Vec<Result<Expression, String>>,
    ) -> Result<Expression, String> {
        match operator.arity() {
            Size::Finite(arity) => {
                if operands.len() != arity {
                    return Err(format!(
                        "Operator {} expects {} operands, but {} were provided",
                        operator,
                        arity,
                        operands.len()
                    ));
                }
            }
            Size::Infinite => {
                if operands.len() < 2 {
                    return Err(format!(
                        "Operator {} expects at least 2 operands, but {} were provided",
                        operator,
                        operands.len()
                    ));
                }
            }
        }
        for operand in &operands {
            if operand.is_err() {
                return Err(operand.as_ref().unwrap().to_string());
            }
        }
        Ok(Expression::OperatorExpression(OperatorExpression {
            operator,
            operands: operands
                .into_iter()
                .map(|operand| operand.unwrap())
                .collect(),
        }))
    }

    fn simplify(&mut self) {
        match self {
            Expression::OperatorExpression(operator_expression) => {
                match operator_expression.operator {
                    Operator::Negation => self.double_negation(),
                    Operator::Addition => {
                        self.merge_addition();
                        self.sum_up();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // called on negations, if the child is also a negation, the child moves up and the parent is deleted
    fn double_negation(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            if let Expression::OperatorExpression(operand) = &mut operator_expression.operands[0] {
                if operand.operator == Operator::Negation {
                    *self = operand.operands[0].clone();
                }
            }
        }
    }

    // called on additions, if the child is also an additive expression, the children are merged
    fn merge_addition(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            let mut operands = Vec::<Expression>::new();
            for operand in &operator_expression.operands {
                if let Expression::OperatorExpression(operand) = operand {
                    if operand.operator == Operator::Addition {
                        operands.extend(operand.operands.clone()); // include the children of the child, but not the child itself -> merge
                    } else {
                        operands.push(Expression::OperatorExpression(operand.clone()));
                        // normally add the child expression (e.g multiplication)
                    }
                } else {
                    operands.push(operand.clone()); // normally add the child symbol
                }
            }
            operator_expression.operands.clear();
            operator_expression.operands.extend(operands);
        }
    }

    // called on additions, if it contains multiple symbols with the same address, they are merged
    fn sum_up(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            let mut found_symbols = HashMap::<String, i128>::new();
            let mut operands_to_remove = Vec::<usize>::new();
            for (operand_index, operand) in operator_expression.operands.iter().enumerate() {
                match operand {
                    Expression::Symbol(symbol) => {
                        operands_to_remove.push(operand_index); // remove the symbol from the expression (possibly add it again if v = 1)
                                                                // not just skip it, because 1 might be 1 - 1 + 1 = 1 -> 3 symbols removed, zero added
                        if found_symbols.contains_key(symbol.name.to_string().as_str()) {
                            *found_symbols
                                .get_mut(symbol.name.to_string().as_str())
                                .unwrap() += 1;
                        } else {
                            found_symbols.insert(symbol.name.to_string(), 1);
                        }
                    }
                    Expression::OperatorExpression(operand) => match operand.operator {
                        Operator::Negation => {
                            // -x
                            if let Expression::Symbol(symbol) = &operand.operands[0] {
                                operands_to_remove.push(operand_index);
                                if found_symbols.contains_key(symbol.name.to_string().as_str()) {
                                    *found_symbols
                                        .get_mut(symbol.name.to_string().as_str())
                                        .unwrap() -= 1;
                                } else {
                                    found_symbols.insert(symbol.name.to_string(), -1);
                                }
                            }
                        }
                        Operator::Multiplication => {
                            // x * Constant | Constant * x
                            if operand.operands.len() == 2 {
                                if let Expression::Symbol(symbol) = &operand.operands[0] {
                                    if let Expression::Constant(constant) = &operand.operands[1] {
                                        operands_to_remove.push(operand_index);
                                        if found_symbols
                                            .contains_key(symbol.name.to_string().as_str())
                                        {
                                            *found_symbols
                                                .get_mut(symbol.name.to_string().as_str())
                                                .unwrap() += *constant;
                                        } else {
                                            found_symbols
                                                .insert(symbol.name.to_string(), *constant);
                                        }
                                    }
                                }
                                if let Expression::Symbol(symbol) = &operand.operands[1] {
                                    if let Expression::Constant(constant) = &operand.operands[0] {
                                        operands_to_remove.push(operand_index);
                                        if found_symbols
                                            .contains_key(symbol.name.to_string().as_str())
                                        {
                                            *found_symbols
                                                .get_mut(symbol.name.to_string().as_str())
                                                .unwrap() += *constant;
                                        } else {
                                            found_symbols
                                                .insert(symbol.name.to_string(), *constant);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            for operand_index in operands_to_remove.iter().rev() {
                operator_expression.operands.remove(*operand_index);
            }
            for (symbol_name, value) in found_symbols {
                match value {
                    0 => {}
                    1 => operator_expression
                        .operands
                        .push(Expression::Symbol(Symbol::new(symbol_name))),
                    _ => operator_expression
                        .operands
                        .push(Expression::OperatorExpression(OperatorExpression {
                            operator: Operator::Multiplication,
                            operands: vec![
                                Expression::Constant(value),
                                Expression::Symbol(Symbol::new(symbol_name)),
                            ],
                        })),
                }
            }
            if operator_expression.operands.len() == 1 {
                *self = operator_expression.operands[0].clone();
            }
        }
    }

    // TODO: negation(x * const) -> x * -const <- found by sum_up
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Constant(constant) => write!(f, "{}", constant),
            Expression::Symbol(symbol) => write!(f, "{}", symbol),
            Expression::OperatorExpression(operator_expression) => {
                write!(f, "({}", operator_expression.operator)?;
                for operand in &operator_expression.operands {
                    write!(f, " {}", operand)?;
                }
                write!(f, ")")
            }
        }
    }
}

// macro to simplify creation of expressions
macro_rules! expr {
    ($symbol:expr) => {
        Ok(Expression::Symbol($symbol.clone()))
    };
    ($operator:ident, $operand:expr) => {
        Expression::new(Operator::$operator, vec![$operand])
    };
    ($operator:ident, $operand1:expr, $operand2:expr) => {
        Expression::new(Operator::$operator, vec![$operand1, $operand2])
    };
    ($operator:ident, $operand1:expr, $operand2:expr, $operand3:expr) => {
        Expression::new(Operator::$operator, vec![$operand1, $operand2, $operand3])
    };
    ($operator:ident, $operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr) => {
        Expression::new(
            Operator::$operator,
            vec![$operand1, $operand2, $operand3, $operand4],
        )
    };
    ($operator:ident, $operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr) => {
        Expression::new(
            Operator::$operator,
            vec![$operand1, $operand2, $operand3, $operand4, $operand5],
        )
    };
}

macro_rules! number {
    ($el:expr) => {
        Ok(Expression::Constant($el))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let x = Symbol::new("x".to_string());
        let expr = expr!(
            Negation,
            expr!(
                Addition,
                expr!(Multiplication, expr!(x), expr!(x)),
                expr!(x)
            )
        );
        assert_eq!(format!("{}", expr.unwrap()), "(- (+ (* x x) x))");
    }

    #[test]
    fn test_double_negation() {
        let x = Symbol::new("x".to_string());
        let mut expr = expr!(Negation, expr!(Negation, expr!(x)));
        if let Result::Ok(expr) = &mut expr {
            expr.simplify();
        }
        assert_eq!(format!("{}", expr.unwrap()), "x");
        let mut expr = expr!(Negation, expr!(Negation, expr!(Negation, expr!(x))));
        if let Result::Ok(expr) = &mut expr {
            expr.simplify();
        }
        assert_eq!(format!("{}", expr.unwrap()), "(- x)");
    }

    #[test]
    fn merge_addition() {
        let x = Symbol::new("x".to_string());
        let mut expr = expr!(Addition, expr!(Addition, expr!(x), expr!(x)), expr!(x));
        if let Result::Ok(expr) = &mut expr {
            expr.simplify();
        }
        assert_eq!(format!("{}", expr.unwrap()), "(* 3 x)");
        // x + 2 * x - 7 * x + y + 2 * y + 3 * y
        let y = Symbol::new("y".to_string());
        let mut expr = expr!(
            Addition,
            expr!(
                Addition,
                expr!(x),
                expr!(Multiplication, number!(2), expr!(x))
            ),
            expr!(Negation, expr!(Multiplication, number!(7), expr!(x))),
            expr!(y),
            expr!(Multiplication, number!(2), expr!(y)),
            expr!(Multiplication, number!(3), expr!(y))
        );
        if let Result::Ok(expr) = &mut expr {
            expr.simplify();
        }
        assert_eq!(format!("{}", expr.unwrap()), "(+ (* -4 x) (* 6 y))");
    }
}

//                 Operator::Addition => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Addition {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands
//                                     .push(Expression::OperatorExpression((*operand).clone()));
//                             }
//                         } else {
//                             operands.push(operand.clone());
//                         }
//                     }
//                     *self = Expression::new(Operator::Addition, operands).unwrap();
//                 }
//                 Operator::Subtraction => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Subtraction {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands.push(operand.clone());
//                             }
//                         } else {
//                             operands.push(operand.clone());
//                         }
//                     }
//                     *self = Expression::new(Operator::Subtraction, operands).unwrap();
//                 }
//                 Operator::Multiplication => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Multiplication {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands.push(operand.clone());
//                             }
//                         } else {
//                             operands.push(operand.clone());
//                         }
//                     }
//                     *self = Expression::new(Operator::Multiplication, operands).unwrap();
//                 }
//                 Operator::Division => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Division {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands.push(operand.clone());
//                             }
//                         } else {
//                             operands.push(
//                                 Expression::new(
//                                     Operator::Exponentiation,
//                                     vec![
//                                         operand.clone(),
//                                         Expression::new(
//                                             Operator::Negation,
//                                             vec![expr!(2).unwrap()],
//                                         )
//                                         .unwrap(),
//                                     ],
//                                 )
//                                 .unwrap(),
//                             );
//                         }
//                     }
//                     *self = Expression::new(Operator::Division, operands).unwrap();
//                 }
//                 Operator::Exponentiation => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Exponentiation {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands.push(operand.clone());
//                             }
//                         } else {
//                             operands.push(operand.clone());
//                         }
//                     }
//                     *self = Expression::new(Operator::Exponentiation, operands).unwrap();
//                 }
//                 Operator::Logarithm => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Logarithm {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands.push(operand.clone());
//                             }
//                         } else {
//                             operands.push(operand.clone());
//                         }
//                     }
//                     *self = Expression::new(Operator::Logarithm, operands).unwrap();
//                 }
//                 Operator::Root => {
//                     let mut operands = Vec::new();
//                     for operand in &operator_expression.operands {
//                         if let Expression::OperatorExpression(operand) = operand {
//                             if operand.operator == Operator::Root {
//                                 operands.extend(operand.operands.clone());
//                             } else {
//                                 operands.push(operand.clone());
//                             }
//                         } else {
//                             operands.push(operand.clone());
//                         }
//                     }
//                     *self = Expression::new(Operator::Root, operands).unwrap();
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
