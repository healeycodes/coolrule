use pom::parser::*;
use std::str::{self, FromStr};

#[derive(Debug)]
pub enum BinOp {
    Equal,              // =, ==, eq
    NotEqual,           // !=, ne, ≠
    GreaterThan,        // >, gt
    GreaterThanOrEqual, // >=, ge, ≥
    LessThan,           // <, lt
    LessThanOrEqual,    // <=, le, ≤
    In,                 // in, ∈
    NotIn,              // notin, ∉
    Is,                 // is
    IsNot,              // isnot
    SubSetOf,           // ⊆
    SuperSetOf,         // ⊇
    IntersectionOf,     // ∩
    NotIntersectionOf,  // not∩
}

#[derive(Debug, Clone)]
pub enum SimpleValue {
    Number(f64),
    Str(String),
    Bool(bool),
    None,
    // The path to a context value
    // e.g. `foo.bar` -> [`foo`, `bar`]
    PropertyPath(Vec<String>),
}

#[derive(Debug)]
pub enum PropertyVal {
    SimpleValue(SimpleValue),
    Group(Vec<SimpleValue>),
}

#[derive(Debug)]
pub enum BooleanCondition {
    Comparison(PropertyVal, BinOp, PropertyVal),
    Group(Box<BooleanExpression>),
}

#[derive(Debug)]
pub enum AndOr {
    And,
    Or,
}

#[derive(Debug)]
pub struct BooleanExpression {
    pub initial: BooleanCondition,
    pub conditions: Vec<(AndOr, BooleanCondition)>,
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn property_path<'a>() -> Parser<'a, u8, Vec<Vec<u8>>> {
    let ascii = one_of(b"_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    list(ascii.repeat(1..), sym(b'.'))
}

fn lparen<'a>() -> Parser<'a, u8, ()> {
    seq(b"(").discard()
}

fn rparen<'a>() -> Parser<'a, u8, ()> {
    seq(b")").discard()
}

fn binary_op<'a>() -> Parser<'a, u8, BinOp> {
    (seq(b"==") | seq(b"=") | seq(b"eq")).map(|_| BinOp::Equal)
        | (seq(b"!=") | seq(b"ne") | seq("≠".as_bytes())).map(|_| BinOp::NotEqual)
        | (seq(b">=") | seq(b"ge") | seq("≥".as_bytes())).map(|_| BinOp::GreaterThanOrEqual)
        | (seq(b">") | seq(b"gt")).map(|_| BinOp::GreaterThan)
        | (seq(b"<=") | seq(b"le") | seq("≤".as_bytes())).map(|_| BinOp::LessThanOrEqual)
        | (seq(b"<") | seq(b"lt")).map(|_| BinOp::LessThan)
        | (seq(b"in") | seq("∈".as_bytes())).map(|_| BinOp::In)
        | (seq(b"notin") | seq("∉".as_bytes())).map(|_| BinOp::NotIn)
        | seq(b"isnot").map(|_| BinOp::IsNot)
        | seq(b"is").map(|_| BinOp::Is)
        | seq("⊆".as_bytes()).map(|_| BinOp::SubSetOf)
        | seq("⊇".as_bytes()).map(|_| BinOp::SuperSetOf)
        | seq("∩".as_bytes()).map(|_| BinOp::IntersectionOf)
        | seq("not∩".as_bytes()).map(|_| BinOp::NotIntersectionOf)
}

fn real_number<'a>() -> Parser<'a, u8, f64> {
    let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
    let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
    let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| f64::from_str(&s))
}

fn integer<'a>() -> Parser<'a, u8, u8> {
    one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0')
}

fn str<'a>() -> Parser<'a, u8, String> {
    (sym(b'"') * none_of(b"\"").repeat(0..) - sym(b'"')).convert(String::from_utf8)
}

fn bool<'a>() -> Parser<'a, u8, SimpleValue> {
    ((seq(b"t") | seq(b"T"))
        + (seq(b"r") | seq(b"R"))
        + (seq(b"u") | seq(b"U"))
        + (seq(b"e") | seq(b"E")))
    .map(|_| SimpleValue::Bool(true))
        | ((seq(b"f") | seq(b"F"))
            + (seq(b"a") | seq(b"A"))
            + (seq(b"l") | seq(b"L"))
            + (seq(b"s") | seq(b"S"))
            + (seq(b"e") | seq(b"E")))
        .map(|_| SimpleValue::Bool(true))
}

fn none<'a>() -> Parser<'a, u8, u8> {
    ((seq(b"n") | seq(b"N"))
        + (seq(b"o") | seq(b"O"))
        + (seq(b"n") | seq(b"N"))
        + (seq(b"e") | seq(b"E")))
    .map(|_| 0)
}

fn simple_value<'a>() -> Parser<'a, u8, SimpleValue> {
    space()
        * (real_number().map(|f| SimpleValue::Number(f))
            | integer().map(|i| SimpleValue::Number(i.into()))
            | str().map(|s| SimpleValue::Str(s))
            | bool()
            | none().map(|_| SimpleValue::None)
            | property_path().map(|p| {
                SimpleValue::PropertyPath(
                    p.iter()
                        .map(|byte_vec| String::from_utf8_lossy(byte_vec).into_owned())
                        .collect(),
                )
            }))
        - space()
}

fn property_val<'a>() -> Parser<'a, u8, PropertyVal> {
    space()
        * ((lparen() * list(simple_value(), sym(b',') * space()) - rparen())
            .map(|g| PropertyVal::Group(g))
            | simple_value().map(|s| PropertyVal::SimpleValue(s)))
        - space()
}

fn and<'a>() -> Parser<'a, u8, u8> {
    ((seq(b"a") | seq(b"A")) + (seq(b"n") | seq(b"N")) + (seq(b"d") | seq(b"D"))).map(|_| 0)
}

fn or<'a>() -> Parser<'a, u8, u8> {
    ((seq(b"o") | seq(b"O")) + (seq(b"r") | seq(b"R"))).map(|_| 0)
}

fn and_or<'a>() -> Parser<'a, u8, AndOr> {
    and().map(|_| AndOr::And) | or().map(|_| AndOr::Or)
}

fn boolean_condition<'a>() -> Parser<'a, u8, BooleanCondition> {
    space()
        * ((property_val() + binary_op() + property_val())
            .map(|((lval, bin_op), rval)| BooleanCondition::Comparison(lval, bin_op, rval))
            | (lparen() * call(boolean_expression) - rparen()).map(|boolean_expression| {
                BooleanCondition::Group(Box::new(boolean_expression))
            }))
        - space()
}

fn boolean_expression<'a>() -> Parser<'a, u8, BooleanExpression> {
    (boolean_condition() + (and_or()) + call(boolean_expression)).map(
        |((boolean_condition, and_or_initial), boolean_expression)| BooleanExpression {
            initial: boolean_condition,
            conditions: vec![(
                and_or_initial,
                BooleanCondition::Group(Box::new(boolean_expression)),
            )],
        },
    ) | boolean_condition().map(|boolean_condition| BooleanExpression {
        initial: boolean_condition,
        conditions: vec![],
    })
}

pub fn parse<'a>(input: &str) -> Result<BooleanExpression, pom::Error> {
    (space() * boolean_expression() - end()).parse(input.as_bytes())
}

#[test]
fn test_parse() {
    let valid_exprs = [
        "5 > 3",
        "3.5 >= 5",
        "true == true",
        "true == True",
        "false == False",
        "None is None",
        "5 > 3 and 3 > 1",
        "(1=1 or 2=2) and (3 = 3)",
        "foo = \"bar\" AND baz > 10",
        "foo = \"bar\" OR baz > 10",
        "foo.bar = \"bar\"",
        "foo.bar isnot none",
        "x in (5, 6, 7)",
        "(3, 4) not∩ (3, 4, 5)",
    ];

    let mut pass = true;
    for expr in valid_exprs.iter() {
        match parse(expr) {
            Ok(_) => (),
            Err(e) => {
                println!("{expr}");
                match e {
                    pom::Error::Mismatch { message, position } => {
                        let spaces = " ".repeat(position);
                        println!("{} {message}", format!("{}{}", spaces, "^"))
                    }
                    _ => {
                        println!("{e}")
                    }
                }
                pass = false;
            }
        };
    }
    assert!(pass);
}
