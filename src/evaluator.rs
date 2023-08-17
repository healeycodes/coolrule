use crate::parser::{AndOr, BinOp, BooleanCondition, BooleanExpression, PropertyVal, SimpleValue};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
    hash::Hash,
    hash::Hasher,
};

fn get_context_value(
    key: Vec<String>,
    context: &HashMap<Vec<String>, SimpleValue>,
) -> Result<SimpleValue, EvalError> {
    match context.get(&key) {
        Some(v) => match v {
            SimpleValue::PropertyPath(_) => Err(EvalError {
                message: format!("property paths shouldn't be in the context dictionary"),
            }),
            _ => Ok(v.to_owned()),
        },
        None => {
            let formatted = key.join(".");
            Err(EvalError {
                message: format!("{formatted} missing from context"),
            })
        }
    }
}

fn eval_boolean_condition(
    boolean_condition: &BooleanCondition,
    context: &HashMap<Vec<String>, SimpleValue>,
) -> Result<bool, EvalError> {
    match boolean_condition {
        BooleanCondition::Comparison(lval, bin_op, rval) => match (lval, rval) {
            (PropertyVal::SimpleValue(_sv1), PropertyVal::SimpleValue(_sv2)) => {
                let sv1: SimpleValue = match _sv1 {
                    SimpleValue::PropertyPath(p) => get_context_value(p.clone(), context)?,
                    _ => _sv1.clone(),
                };
                let sv2: SimpleValue = match _sv2 {
                    SimpleValue::PropertyPath(p) => get_context_value(p.clone(), context)?,
                    _ => _sv2.clone(),
                };
                match bin_op {
                    BinOp::Equal => Ok(sv1 == sv2),
                    BinOp::NotEqual => Ok(sv1 != sv2),
                    BinOp::GreaterThan => Ok(sv1.partial_cmp(&sv2) == Some(Ordering::Greater)),
                    BinOp::GreaterThanOrEqual => {
                        Ok(sv1 == sv2 || sv1.partial_cmp(&sv2) == Some(Ordering::Greater))
                    }
                    BinOp::LessThan => Ok(sv1.partial_cmp(&sv2) == Some(Ordering::Less)),
                    BinOp::LessThanOrEqual => {
                        Ok(sv1 == sv2 || sv1.partial_cmp(&sv2) == Some(Ordering::Less))
                    }
                    BinOp::In => Err(EvalError {
                        message: format!("{sv2} is not iterable"),
                    }),
                    BinOp::NotIn => Err(EvalError {
                        message: format!("{sv2} is not iterable"),
                    }),
                    BinOp::Is => Ok(sv1 == sv2),
                    BinOp::IsNot => Ok(sv1 != sv2),
                    BinOp::SubSetOf => Err(EvalError {
                        message: format!("{sv2} is not iterable"),
                    }),
                    BinOp::SuperSetOf => Err(EvalError {
                        message: format!("{sv2} is not iterable"),
                    }),
                    BinOp::IntersectionOf => Err(EvalError {
                        message: format!("{sv2} is not iterable"),
                    }),
                    BinOp::NotIntersectionOf => Err(EvalError {
                        message: format!("{sv2} is not iterable"),
                    }),
                }
            }
            (PropertyVal::SimpleValue(_sv), PropertyVal::Group(_gv)) => {
                let sv: SimpleValue = match _sv {
                    SimpleValue::PropertyPath(p) => get_context_value(p.clone(), context)?,
                    _ => _sv.clone(),
                };
                let mut gv: Vec<SimpleValue> = vec![];
                for v in _gv.iter() {
                    gv.push(match v {
                        SimpleValue::PropertyPath(p) => get_context_value(p.to_vec(), context)?,
                        _ => v.clone(),
                    })
                }
                match bin_op {
                    BinOp::Equal => Ok(false),
                    BinOp::NotEqual => Ok(true),
                    BinOp::GreaterThan => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::GreaterThanOrEqual => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::LessThan => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::LessThanOrEqual => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::In => {
                        for v in gv {
                            if sv == v {
                                return Ok(true);
                            }
                        }
                        return Ok(false);
                    }
                    BinOp::NotIn => {
                        for v in gv {
                            if sv == v {
                                return Ok(false);
                            }
                        }
                        return Ok(true);
                    }
                    BinOp::Is => Ok(false),
                    BinOp::IsNot => Ok(true),
                    BinOp::SubSetOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::SuperSetOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::IntersectionOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::NotIntersectionOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                }
            }
            (PropertyVal::Group(_), PropertyVal::SimpleValue(_sv)) => {
                let sv: SimpleValue = match _sv {
                    SimpleValue::PropertyPath(p) => get_context_value(p.clone(), context)?,
                    _ => _sv.clone(),
                };
                match bin_op {
                    BinOp::Equal => Ok(false),
                    BinOp::NotEqual => Ok(true),
                    BinOp::GreaterThan => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::GreaterThanOrEqual => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::LessThan => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::LessThanOrEqual => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::In => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::NotIn => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::Is => Ok(false),
                    BinOp::IsNot => Ok(true),
                    BinOp::SubSetOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::SuperSetOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::IntersectionOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                    BinOp::NotIntersectionOf => Err(EvalError {
                        message: format!("{sv} is not iterable"),
                    }),
                }
            }
            (PropertyVal::Group(_gv1), PropertyVal::Group(_gv2)) => {
                let mut gv1: Vec<SimpleValue> = vec![];
                for v in _gv1.iter() {
                    gv1.push(match v {
                        SimpleValue::PropertyPath(p) => get_context_value(p.to_vec(), context)?,
                        _ => v.clone(),
                    })
                }
                let mut gv2: Vec<SimpleValue> = vec![];
                for v in _gv2.iter() {
                    gv2.push(match v {
                        SimpleValue::PropertyPath(p) => get_context_value(p.to_vec(), context)?,
                        _ => v.clone(),
                    })
                }
                match bin_op {
                    BinOp::Equal => {
                        if gv1.len() != gv2.len() {
                            return Ok(false);
                        }
                        for i in 0..gv1.len() {
                            if gv1[i] != gv2[i] {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    BinOp::NotEqual => {
                        if gv1.len() != gv2.len() {
                            return Ok(true);
                        }
                        for i in 0..gv1.len() {
                            if gv1[i] != gv2[i] {
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    }
                    BinOp::GreaterThan => {
                        for i in 0..usize::min(gv1.len(), gv2.len()) {
                            if !(gv1[i] > gv2[i]) {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    BinOp::GreaterThanOrEqual => {
                        for i in 0..usize::min(gv1.len(), gv2.len()) {
                            if !(gv1[i] > gv2[i] || gv1[i] == gv2[i]) {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    BinOp::LessThan => {
                        for i in 0..usize::min(gv1.len(), gv2.len()) {
                            if !(gv1[i] < gv2[i]) {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    BinOp::LessThanOrEqual => {
                        for i in 0..usize::min(gv1.len(), gv2.len()) {
                            if !(gv1[i] < gv2[i] || gv1[i] == gv2[i]) {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    BinOp::In => Ok(false),
                    BinOp::NotIn => Ok(true),
                    BinOp::Is => {
                        if gv1.len() != gv2.len() {
                            return Ok(false);
                        }
                        for i in 0..gv1.len() {
                            if gv1[i] != gv2[i] {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    BinOp::IsNot => {
                        if gv1.len() != gv2.len() {
                            return Ok(true);
                        }
                        for i in 0..gv1.len() {
                            if gv1[i] != gv2[i] {
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    }
                    BinOp::SubSetOf => Ok(is_subset(&gv1, &gv2)),
                    BinOp::SuperSetOf => Ok(is_super_set(&gv1, &gv2)),
                    BinOp::IntersectionOf => Ok(intersection_of(&gv1, &gv2)),
                    BinOp::NotIntersectionOf => Ok(not_intersection_of(&gv1, &gv2)),
                }
            }
        },
        BooleanCondition::Group(boxed_expr) => {
            eval_boolean_expression(&*boxed_expr, context)
        }
    }
}

fn eval_boolean_expression(
    boolean_expression: &BooleanExpression,
    context: &HashMap<Vec<String>, SimpleValue>,
) -> Result<bool, EvalError> {
    let mut result = eval_boolean_condition(&boolean_expression.initial, context)?;
    for (and_or, cond) in boolean_expression.conditions.as_slice() {
        let next = eval_boolean_condition(&cond, context)?;
        match and_or {
            AndOr::And => {
                result = result && next;
            }
            AndOr::Or => {
                result = result || next;
            }
        }
    }
    return Ok(result);
}

pub fn eval(boolean_expression: &BooleanExpression) -> Result<bool, EvalError> {
    return eval_boolean_expression(&boolean_expression, &HashMap::new());
}

pub fn eval_with_context(
    boolean_expression: &BooleanExpression,
    context: &HashMap<Vec<String>, SimpleValue>,
) -> Result<bool, EvalError> {
    return eval_boolean_expression(&boolean_expression, &context);
}

#[test]
fn test_eval() {
    let exprs = [
        ("5 > 3", true),
        ("5 < 3", false),
        ("5 > 5", false),
        ("3 >= 5", false),
        ("5 >= 3", true),
        ("5 >= 5", true),
        ("5 <= 3", false),
        ("3 <= 5", true),
        ("3 <= 5", true),
        ("5 ≥ 3", true),
        ("5 ≥ 5", true),
        ("3 ≤ 3", true),
        ("3 ≤ 5", true),
        ("7 == true", false),
        ("true == true", true),
        ("none is none", true),
        ("1 != 2", true),
        ("1 != 1", false),
        ("2 != true", true),
        ("1 ≠ 2", true),
        ("1 ≠ 1", false),
        ("2 ≠ true", true),
        ("5 > 3 and 3 > 1", true),
        ("5 > 3 and 3 > 5", false),
        ("5 > 3 or 3 > 5", true),
        ("5 > 3 and (3 > 5 or 3 > 1)", true),
        ("5 > 3 and (3 > 5 and 3 < 1)", false),
        ("(1=1 or 2=2) and (3 = 3)", true),
        ("(1=1 or 2=2) and (3 = 4)", false),
        ("(1, 2, 3) ⊆ (1, 2, 3)", true),
        ("(1, 2, 3) ⊇ (1, 2, 3)", true),
        ("(1, 2, 3) ⊆ (1, 2, 3, 4)", true),
        ("(1, 2, 3, 4) ⊇ (1, 2, 3)", true),
        ("(1, 2, 3) ⊆ (1, 2)", false),
        ("(1, 2) ⊇ (1, 2, 3)", false),
        ("(1, 2, 3) ∩ (1, 2, 3)", true),
        ("(4) ∩ (3, 4, 5)", true),
        ("(1, 2, 3) ∩ (4, 5, 6)", false),
        ("(4) not∩ (1, 2, 3)", true),
        ("(1, 2) not∩ (4, 5, 6)", true),
        ("(3) not∩ (3, 4, 5)", false),
        ("(3, 4) not∩ (3, 4, 5)", false),
    ];
    let exprs_with_context = [
        (
            "foo = \"bar\" and baz > 10",
            vec![
                ("foo", SimpleValue::Str("bar".to_owned())),
                ("baz", SimpleValue::Number(20.0)),
            ],
            true,
        ),
        (
            "foo = \"bar\" and baz > 10",
            vec![
                ("foo", SimpleValue::Str("bar".to_owned())),
                ("baz", SimpleValue::Number(9.0)),
            ],
            false,
        ),
        (
            "foo.bar = \"bar\"",
            vec![("foo.bar", SimpleValue::Str("bar".to_owned()))],
            true,
        ),
        (
            "foo.bar.zoo isnot none and true is true",
            vec![("foo.bar.zoo", SimpleValue::Number(4.0))],
            true,
        ),
        (
            "x in (5, 6, 7)",
            vec![("x", SimpleValue::Number(5.0))],
            true,
        ),
        ("x ∈ (5, 6, 7)", vec![("x", SimpleValue::Number(5.0))], true),
        (
            "x ∉ (5, 6, 7)",
            vec![("x", SimpleValue::Number(5.0))],
            false,
        ),
        ("(a) == (a)", vec![("a", SimpleValue::Number(5.0))], true),
        ("(a) == 1", vec![("a", SimpleValue::Number(5.0))], false),
        ("1 == (a)", vec![("a", SimpleValue::Number(5.0))], false),
    ];

    for (expr, test) in exprs.iter() {
        let boolean_expression = crate::parser::parse(expr).unwrap();
        let result = eval(&boolean_expression);
        assert!(result.unwrap() == *test, "{expr} should eval to {test}");
    }
    for (expr, ctx, test) in exprs_with_context.iter() {
        let boolean_expression = crate::parser::parse(expr).unwrap();
        let mut context: HashMap<Vec<String>, SimpleValue> = HashMap::new();
        for (k, v) in ctx {
            context.insert(
                k.split('.')
                    .map(|substring| substring.to_string())
                    .collect(),
                v.clone(),
            );
        }
        let result = eval_with_context(&boolean_expression, &context);
        assert!(result.unwrap() == *test, "{expr} should eval to {test}");
    }

    // TODO: add better coverage for expected errors
    match eval(&crate::parser::parse("true = a").unwrap()) {
        Ok(_) => Err("expected error"),
        Err(_) => Ok(()),
    }
    .unwrap();
}

#[derive(Debug)]
pub struct EvalError {
    message: String,
}
impl Error for EvalError {}
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", self.message)
    }
}

impl fmt::Display for SimpleValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleValue::Number(n) => write!(f, "{n}"),
            SimpleValue::Str(s) => write!(f, "{s}"),
            SimpleValue::Bool(b) => write!(f, "{b}"),
            SimpleValue::None => write!(f, "none"),
            SimpleValue::PropertyPath(p) => write!(f, "{p:?}"),
        }
    }
}
impl PartialEq for SimpleValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SimpleValue::Number(n1), SimpleValue::Number(n2)) => n1 == n2,
            (SimpleValue::Str(s1), SimpleValue::Str(s2)) => s1 == s2,
            (SimpleValue::Bool(b1), SimpleValue::Bool(b2)) => b1 == b2,
            (SimpleValue::None, SimpleValue::None) => true,
            _ => false,
        }
    }
}
impl Eq for SimpleValue {}
impl PartialOrd for SimpleValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (SimpleValue::Number(num1), SimpleValue::Number(num2)) => num1.partial_cmp(num2),
            (SimpleValue::Str(str1), SimpleValue::Str(str2)) => str1.partial_cmp(str2),
            (SimpleValue::Bool(bool1), SimpleValue::Bool(bool2)) => bool1.partial_cmp(bool2),
            (SimpleValue::None, SimpleValue::None) => Some(Ordering::Equal),
            _ => None,
        }
    }
}
impl Hash for SimpleValue {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            SimpleValue::Number(num) => {
                hasher.write_u64(num.to_bits());
            }
            SimpleValue::Str(str) => {
                str.hash(hasher);
            }
            SimpleValue::Bool(bool_val) => {
                bool_val.hash(hasher);
            }
            SimpleValue::None => hasher.write_u64(0),
            SimpleValue::PropertyPath(_) => panic!("property paths can't be hashed"),
        }
    }
}

fn is_subset<T: Eq + std::hash::Hash>(subset: &Vec<T>, superset: &Vec<T>) -> bool {
    let superset_set: HashSet<_> = superset.iter().collect();
    subset.iter().all(|item| superset_set.contains(item))
}

fn is_super_set<T: Eq + std::hash::Hash>(superset: &Vec<T>, subset: &Vec<T>) -> bool {
    is_subset(subset, superset)
}

fn intersection_of<T: Eq + std::hash::Hash>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    let set1: HashSet<_> = vec1.iter().collect();
    let set2: HashSet<_> = vec2.iter().collect();
    set1.intersection(&set2).count() > 0
}

fn not_intersection_of<T: Eq + std::hash::Hash>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    let set1: HashSet<_> = vec1.iter().collect();
    let set2: HashSet<_> = vec2.iter().collect();
    set1.intersection(&set2).count() == 0
}
