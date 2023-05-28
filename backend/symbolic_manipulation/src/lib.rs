#![allow(dead_code)]
#![allow(unused_macros)]
use core::fmt;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
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

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Operator {
    Negation,
    Reciprocal,
    Factorial,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Logarithm,
    Root,
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Size {
    Finite(usize),
    Infinite,
}

impl Operator {
    fn arity(&self) -> Size {
        match self {
            Operator::Negation | Operator::Factorial | Operator::Reciprocal => Size::Finite(1),
            Operator::Addition
            | Operator::Subtraction
            | Operator::Multiplication
            | Operator::Division => Size::Infinite,
            Operator::Exponentiation | Operator::Logarithm | Operator::Root => Size::Finite(2),
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Negation | Operator::Reciprocal | Operator::Factorial => 4,
            Operator::Exponentiation => 3,
            Operator::Logarithm | Operator::Root => 2,
            Operator::Multiplication | Operator::Division => 1,
            Operator::Addition | Operator::Subtraction => 0,
        }
    }

    fn is_left_associative(&self) -> bool {
        matches!(
            self,
            Operator::Addition
                | Operator::Subtraction
                | Operator::Multiplication
                | Operator::Division
        )
    }

    fn is_right_associative(&self) -> bool {
        !self.is_left_associative()
    }

    fn is_commutative(&self) -> bool {
        matches!(self, Operator::Addition | Operator::Multiplication)
    }

    fn is_associative(&self) -> bool {
        matches!(self, Operator::Addition | Operator::Multiplication)
    }

    fn is_distributive_under(&self) -> Option<Operator> {
        match self {
            Operator::Addition => Some(Operator::Multiplication),
            Operator::Multiplication => Some(Operator::Exponentiation),
            _ => None,
        }
    }

    fn neutral_element(&self) -> Option<Expression> {
        match self {
            Operator::Addition => Some(Expression::Constant(0)),
            Operator::Multiplication => Some(Expression::Constant(1)),
            _ => None,
        }
    }

    fn inverse(&self) -> Option<Operator> {
        match self {
            Operator::Addition => Some(Operator::Negation),
            Operator::Multiplication => Some(Operator::Reciprocal),
            _ => None,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Negation => write!(f, "-"),
            Operator::Reciprocal => write!(f, "1/"),
            Operator::Factorial => write!(f, "!"),
            Operator::Addition => write!(f, "+"),
            Operator::Subtraction => write!(f, "-"), // inexistent
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Exponentiation => write!(f, "^"),
            Operator::Logarithm => write!(f, "log"),
            Operator::Root => write!(f, "root"),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct OperatorExpression {
    operator: Operator,
    operands: Vec<Expression>,
}

impl OperatorExpression {
    pub fn new(
        operator: Operator,
        operands: Vec<Result<Expression, String>>,
    ) -> Result<OperatorExpression, String> {
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
        operands
            .into_iter()
            .collect::<Result<Vec<Expression>, String>>() // convert Vec<Result> to Result<Vec>
            .map(|result_operands| OperatorExpression {
                operator,
                operands: result_operands,
            }) // map Result<Vec> to Result<OperatorExpression>
    }
}

impl fmt::Display for OperatorExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.operator)?;
        let mut iter = self.operands.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, ", {}", item)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Expression {
    Constant(i128),
    Symbol(Symbol),
    OperatorExpression(OperatorExpression),
}

pub trait ConstructExpression {
    fn construct_expression(&self) -> Result<Expression, String>;
}

impl ConstructExpression for i128 {
    fn construct_expression(&self) -> Result<Expression, String> {
        Ok(Expression::Constant(*self))
    }
}

impl ConstructExpression for Symbol {
    fn construct_expression(&self) -> Result<Expression, String> {
        Ok(Expression::Symbol(self.clone()))
    }
}

impl ConstructExpression for Result<OperatorExpression, String> {
    fn construct_expression(&self) -> Result<Expression, String> {
        self.clone().map(Expression::OperatorExpression)
    }
}

trait InlinePush<T> {
    fn inline_push(&mut self, item: T) -> &mut Self;
}

impl<T> InlinePush<T> for Vec<T> {
    fn inline_push(&mut self, item: T) -> &mut Self {
        self.push(item);
        self
    }
}

trait SafeAdd<K, V> {
    fn safe_add(&mut self, key: K, value: V);
}

impl<K, V> SafeAdd<K, V> for HashMap<K, V>
where
    K: Copy + std::cmp::Eq + std::hash::Hash,
    V: std::ops::Add<Output = V> + std::ops::Sub<Output = V> + Copy + std::ops::AddAssign<V>,
{
    fn safe_add(&mut self, key: K, value: V) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.entry(key) {
            e.insert(value);
        } else {
            *self.get_mut(&key).unwrap() += value;
        }
    }
}

impl Expression {
    fn is_equal(&mut self, other: &mut Expression) -> bool {
        other.simplify();
        format!("{}", self) == format!("{}", other)
    }

    pub fn simplify(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            println!("simplifying {}", operator_expression);
            for operand in &mut operator_expression.operands {
                operand.simplify();
            }
            match operator_expression.operator {
                Operator::Negation => {
                    self.double_negation();
                }
                Operator::Subtraction => {
                    self.remove_subtraction();
                }
                Operator::Addition | Operator::Multiplication => {
                    self.remove_neutral_element();
                    self.merge();
                    self.factor_out();
                }
                _ => {}
            }
        }
    }

    fn remove_subtraction(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            if let Result::Ok(expression) = OperatorExpression::new(
                Operator::Addition,
                vec![
                    Ok(operator_expression.operands[0].clone()),
                    OperatorExpression::new(
                        Operator::Negation,
                        vec![OperatorExpression::new(
                            Operator::Addition,
                            operator_expression.operands[1..]
                                .iter()
                                .cloned()
                                .map(Ok)
                                .collect::<Vec<Result<Expression, String>>>()
                                .inline_push(Ok(Expression::Constant(0)))
                                .clone(),
                        )
                        .construct_expression()],
                    )
                    .construct_expression(),
                ],
            )
            .construct_expression()
            {
                *self = expression;
            }
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

    fn remove_neutral_element(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            if let Some(neutral_element) = operator_expression.operator.neutral_element() {
                operator_expression
                    .operands
                    .retain(|operand| *operand != neutral_element);
            }
        }
    }

    // called on associative operators, if the child is of the same type, the children are merged
    fn merge(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            let mut operands = Vec::<Expression>::new();
            for operand in &operator_expression.operands {
                if let Expression::OperatorExpression(operand) = operand {
                    if operand.operator == operator_expression.operator {
                        operands.extend(operand.operands.clone()); // include the children of the child, but not the child itself -> merge
                    } else {
                        operands.push(Expression::OperatorExpression(operand.clone()));
                        // normally add the child expression
                    }
                } else {
                    operands.push(operand.clone()); // normally add the child symbol
                }
            }
            operator_expression.operands.clear();
            operator_expression.operands.extend(operands);
        }
    }

    fn factor_out(&mut self) {
        if let Expression::OperatorExpression(operator_expression) = self {
            if let Some(distributive_operator) =
                operator_expression.operator.is_distributive_under()
            {
                let mut found_expressions = HashMap::<&Expression, i128>::new();
                for operand in operator_expression.operands.iter() {
                    if let Expression::OperatorExpression(operand) = operand {
                        if let Some(inverse_operator) = operand.operator.inverse() {
                            // first check if the operand is (inverse_operator expression)
                            if operand.operands.len() != 1 {
                                continue;
                            }
                            if operand.operator != inverse_operator {
                                continue;
                            }
                            // -> HashMap[expression] -= 1
                            // e.g. x - x -> 0
                            found_expressions.safe_add(&operand.operands[0], -1);
                            continue;
                        }

                        // first check if the operand is (distributive_operator constant expression)
                        if operand.operator != distributive_operator {
                            continue;
                        }
                        // -> HashMap[expression] += constant
                        // e.g. x + 3 * x -> 4 * x
                        if operand.operands.len() != 2 {
                            continue;
                        }
                        if let Expression::Constant(constant) = &operand.operands[0] {
                            found_expressions.safe_add(&operand.operands[1], *constant);
                        }
                        if let Expression::Constant(constant) = &operand.operands[1] {
                            found_expressions.safe_add(&operand.operands[0], *constant);
                        }
                    }
                    found_expressions.safe_add(operand, 1);
                }
                // the Hash map now contains all expressions and their coefficients
                found_expressions.retain(|_, value| *value != 0);
                // the new operands are (distributive_operator coefficient expression)
                // e.g. x + 4 * x + y -> 5 * x + y
                // or y * y -> y ^ 2
                let new_operands = found_expressions
                    .into_iter()
                    .map(|(expression, coefficient)| {
                        if coefficient == 1 {
                            Ok(expression.clone())
                        } else {
                            OperatorExpression::new(
                                distributive_operator.clone(),
                                vec![Expression::Constant(coefficient), expression.clone()]
                                    .into_iter()
                                    .map(Ok)
                                    .collect::<Vec<Result<Expression, String>>>(),
                            )
                            .construct_expression()
                        }
                    })
                    .collect::<Result<Vec<Expression>, String>>();
                if let Result::Ok(new_operands) = new_operands {
                    if new_operands.len() == 1 {
                        // if n = 1 the operator changes to the distributive operator)
                        *self = new_operands[0].clone();
                    } else {
                        operator_expression.operands = new_operands;
                    }
                }
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

// macro to create trees of symbols and operators
#[macro_export]
macro_rules! sym {
    ($name: expr) => {
        Symbol::new($name.to_string()).construct_expression()
    };
}
#[macro_export]
macro_rules! expr {
    ($constant: expr) => {
        $constant.construct_expression()
    };
    ($operator: ident, $($operand: expr),*) => {
        OperatorExpression::new(
            Operator::$operator,
            vec![
                $($operand),*
            ],
        )
        .construct_expression()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_creation() {
        let mut expression = OperatorExpression::new(
            Operator::Addition,
            vec![
                Symbol::new("x".to_string()).construct_expression(),
                OperatorExpression::new(
                    Operator::Multiplication,
                    vec![
                        2.construct_expression(),
                        Symbol::new("y".to_string()).construct_expression(),
                    ],
                )
                .construct_expression(),
            ],
        )
        .construct_expression();

        if let Result::Ok(expression) = &mut expression {
            assert_eq!(format!("{}", expression), "(+ x (* 2 y))");
        }
    }

    #[test]
    fn test_macro_creation() {
        let mut expression = expr!(
            Addition,
            sym!("x"),
            expr!(Multiplication, expr!(2), sym!("y"))
        );

        if let Result::Ok(expression) = &mut expression {
            assert_eq!(format!("{}", expression), "(+ x (* 2 y))");
        }
    }

    #[test]
    fn test_subtraction() {
        let mut expression = expr!(Subtraction, expr!(2), expr!(3));

        if let Result::Ok(expression) = &mut expression {
            expression.simplify();
            assert_eq!(format!("{}", expression), "(+ 2 (- 3))");
        }
    }

    #[test]
    fn test_double_negation() {
        let mut expression = expr!(
            Negation,
            expr!(
                Negation,
                expr!(Multiplication, expr!(Negation, sym!("x")), sym!("x"))
            )
        );

        if let Result::Ok(expression) = &mut expression {
            expression.simplify();
            assert_eq!(format!("{}", expression), "2");
        }
    }

    #[test]
    fn test_sum_merge() {
        let mut expression = expr!(Addition, sym!("x"), expr!(Addition, sym!("x"), sym!("y")));

        if let Result::Ok(expression) = &mut expression {
            expression.simplify();
            assert_eq!(format!("{}", expression), "(+ (* 2 x) y)");
        }
    }
}
