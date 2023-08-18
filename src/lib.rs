//! A Rust port of [boolrule](https://github.com/tailsdotcom/boolrule/tree/main).
//!
//! This library evaluates boolean expressions.
//!
//! # Usage
//!
//! To use this library, you need to create a `CoolRule` instance by parsing a boolean expression string.
//! The library supports evaluating boolean expressions containing various operations such as `and`, `or`, `not`, comparisons, and set membership checks.
//! Expressions can be evaluated with or without a context, where the context provides values for variables used in the expression.
//!
//! ## Examples
//!
//! Creating and testing an expression without context:
//!
//! ```
//! let expr = coolrule::new("1 > 2 and 3 <= 5").unwrap();
//! let result = expr.test().unwrap(); // false
//! ```
//!
//! Creating and testing an expression with context:
//!
//! ```
//! use coolrule::{Value};
//! use std::collections::HashMap;
//!
//! let expr = coolrule::new("x == 5").unwrap();
//! let mut context = HashMap::new();
//! context.insert(vec!["x"], Value::Number(5.0));
//! let result = expr.test_with_context(&context).unwrap(); // true
//! ```
//!

mod evaluator;
mod parser;

use evaluator::EvalError;
use parser::{BooleanExpression, SimpleValue};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CoolRuleError {
    EvalError(EvalError),
    ParseError(pom::Error),
}

/// Represents possible values that can be used in boolean expressions.
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    None,
}

/// Represents a parsed and processed boolean expression.
pub struct CoolRule {
    boolean_expression: BooleanExpression,
}

/// Creates a new `CoolRule` instance by parsing the given boolean expression string.
///
/// # Arguments
///
/// * `expr` - A string containing the boolean expression to parse.
///
/// # Returns
///
/// A `Result` containing a `CoolRule` instance if parsing is successful, or a `CoolRuleError` if an error occurs during parsing.
pub fn new(expr: &str) -> Result<CoolRule, CoolRuleError> {
    match parse(expr) {
        Ok(boolean_expression) => Ok(CoolRule {
            boolean_expression: boolean_expression,
        }),
        Err(e) => Err(CoolRuleError::ParseError(e)),
    }
}

impl CoolRule {
    /// Evaluates the boolean expression without any context.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating the evaluation result if successful, or a `CoolRuleError` if an error occurs during evaluation.
    pub fn test(&self) -> Result<bool, CoolRuleError> {
        match eval(&self.boolean_expression) {
            Ok(b) => Ok(b),
            Err(e) => Err(CoolRuleError::EvalError(e)),
        }
    }

    /// Evaluates the boolean expression with the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - A hashmap representing the context with variable names as keys and their corresponding values as `Value` enum variants.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating the evaluation result if successful, or a `CoolRuleError` if an error occurs during evaluation.
    pub fn test_with_context(
        &self,
        context: &HashMap<Vec<&str>, Value>,
    ) -> Result<bool, CoolRuleError> {
        let mut ctx: HashMap<Vec<&str>, SimpleValue> = HashMap::new();
        context.iter().for_each(|(k, v)| {
            ctx.insert(
                k.to_vec(),
                match v {
                    Value::Number(n) => SimpleValue::Number(*n),
                    Value::Str(s) => SimpleValue::Str(s.clone()),
                    Value::Bool(b) => SimpleValue::Bool(*b),
                    Value::None => SimpleValue::None,
                },
            );
        });
        match eval_with_context(&self.boolean_expression, &ctx) {
            Ok(b) => Ok(b),
            Err(e) => Err(CoolRuleError::EvalError(e)),
        }
    }
}

use crate::{
    evaluator::{eval, eval_with_context},
    parser::parse,
};

#[test]
fn test_bool_rule_test_suite() {
    let exprs = [
        // Tests ported from boolrule
        ("5 > 3", HashMap::new(), true),
        ("5 < 3", HashMap::new(), false),
        ("5 > 5", HashMap::new(), false),
        ("3 >= 5", HashMap::new(), false),
        ("5 >= 3", HashMap::new(), true),
        ("5 >= 5", HashMap::new(), true),
        ("5 <= 3", HashMap::new(), false),
        ("3 <= 5", HashMap::new(), true),
        ("3 <= 5", HashMap::new(), true),
        ("5 ≥ 3", HashMap::new(), true),
        ("5 ≥ 5", HashMap::new(), true),
        ("3 ≤ 3", HashMap::new(), true),
        ("3 ≤ 5", HashMap::new(), true),
        ("7 == true", HashMap::new(), false),
        ("true == true", HashMap::new(), true),
        ("None is None", HashMap::new(), true),
        ("1 != 2", HashMap::new(), true),
        ("1 != 1", HashMap::new(), false),
        ("2 != true", HashMap::new(), true),
        ("1 ≠ 2", HashMap::new(), true),
        ("1 ≠ 1", HashMap::new(), false),
        ("2 ≠ true", HashMap::new(), true),
        ("5 > 3 and 3 > 1", HashMap::new(), true),
        ("5 > 3 and 3 > 5", HashMap::new(), false),
        ("5 > 3 or 3 > 5", HashMap::new(), true),
        ("5 > 3 and (3 > 5 or 3 > 1)", HashMap::new(), true),
        ("5 > 3 and (3 > 5 and 3 < 1)", HashMap::new(), false),
        ("(1=1 or 2=2) and (3 = 3)", HashMap::new(), true),
        ("(1=1 or 2=2) and (3 = 4)", HashMap::new(), false),
        (
            "foo = \"bar\" AND baz > 10",
            HashMap::from([
                (vec!["foo"], Value::Str("bar".to_string())),
                (vec!["baz"], Value::Number(20.0)),
            ]),
            true,
        ),
        (
            "foo = \"bar\" AND baz > 10",
            HashMap::from([
                (vec!["foo"], Value::Str("bar".to_string())),
                (vec!["baz"], Value::Number(9.0)),
            ]),
            false,
        ),
        (
            "foo = \"bar\" AND (\"a\" = \"b\" OR baz > 10)",
            HashMap::from([
                (vec!["foo"], Value::Str("bar".to_string())),
                (vec!["baz"], Value::Number(11.0)),
            ]),
            true,
        ),
        (
            "foo.bar = \"bar\"",
            HashMap::from([(vec!["foo", "bar"], Value::Str("bar".to_string()))]),
            true,
        ),
        (
            "foo.bar isnot none",
            HashMap::from([(vec!["foo", "bar"], Value::Number(4.0))]),
            true,
        ),
        (
            "foo.bar is none",
            HashMap::from([(vec!["foo", "bar"], Value::None)]),
            true,
        ),
        (
            "foo.bar is none",
            HashMap::from([(vec!["foo", "bar"], Value::None)]),
            true,
        ),
        ("1=1 and 2 in (1, true)", HashMap::new(), false),
        (
            "x in (5, 6, 7)",
            HashMap::from([(vec!["x"], Value::Number(5.0))]),
            true,
        ),
        (
            "x in (5, 6, 7)",
            HashMap::from([(vec!["x"], Value::Number(8.0))]),
            false,
        ),
        (
            "x in (5, 6, 7, y)",
            HashMap::from([
                (vec!["x"], Value::Number(99.0)),
                (vec!["y"], Value::Number(99.0)),
            ]),
            true,
        ),
        (
            "x ∈ (5, 6, 7)",
            HashMap::from([(vec!["x"], Value::Number(5.0))]),
            true,
        ),
        (
            "x ∈ (5, 6, 7)",
            HashMap::from([(vec!["x"], Value::Number(8.0))]),
            false,
        ),
        (
            "x ∈ (5, 6, 7, y)",
            HashMap::from([
                (vec!["x"], Value::Number(99.0)),
                (vec!["y"], Value::Number(99.0)),
            ]),
            true,
        ),
        (
            "x ∉ (5, 6, 7)",
            HashMap::from([(vec!["x"], Value::Number(5.0))]),
            false,
        ),
        (
            "x ∉ (5, 6, 7)",
            HashMap::from([(vec!["x"], Value::Number(8.0))]),
            true,
        ),
        (
            "x ∉ (5, 6, 7, y)",
            HashMap::from([
                (vec!["x"], Value::Number(99.0)),
                (vec!["y"], Value::Number(99.0)),
            ]),
            false,
        ),
        ("(1, 2, 3) ⊆ (1, 2, 3)", HashMap::new(), true),
        ("(1, 2, 3) ⊇ (1, 2, 3)", HashMap::new(), true),
        ("(1, 2, 3) ⊆ (1, 2, 3, 4)", HashMap::new(), true),
        ("(1, 2, 3, 4) ⊇ (1, 2, 3)", HashMap::new(), true),
        ("(1, 2, 3) ⊆ (1, 2)", HashMap::new(), false),
        ("(1, 2) ⊇ (1, 2, 3)", HashMap::new(), false),
        ("(1, 2, 3) ∩ (1, 2, 3)", HashMap::new(), true),
        ("(4) ∩ (3, 4, 5)", HashMap::new(), true),
        ("(1, 2, 3) ∩ (4, 5, 6)", HashMap::new(), false),
        ("(4) not∩ (1, 2, 3)", HashMap::new(), true),
        ("(1, 2) not∩ (4, 5, 6)", HashMap::new(), true),
        ("(3) not∩ (3, 4, 5)", HashMap::new(), false),
        ("(3, 4) not∩ (3, 4, 5)", HashMap::new(), false),
        // coolrule specific tests
        ("(1, 2) == (1, 2)", HashMap::new(), true),
        ("(4, none) >= (1, none)", HashMap::new(), true),
        ("none in (none)", HashMap::new(), true),
    ];

    assert_eq!(new("1 == 1").unwrap().test().unwrap(), true);
    for (expr, ctx, result) in exprs.iter() {
        println!("{}", expr);
        let cr = new(&expr).unwrap();
        assert_eq!(cr.test_with_context(ctx).unwrap(), *result);
    }
}
