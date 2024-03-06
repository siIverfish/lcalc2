// λ

#![feature(box_patterns)]

pub mod error;
pub mod preprocessor;
pub mod spec;

pub mod ds;
pub mod parse;

pub mod run;

// TODO: HashMap or BTreeMap?

use std::{collections::BTreeMap, iter::Peekable, str::Chars};

use ds::Token;
use error::{Error, ParserError};
use preprocessor::preprocess;

use parse::parse_iter;

use run::evaluate;

pub fn parse_file(input: &str) -> Result<Token, ParserError> {
    let (definitions, input) = preprocess(input)?;

    parse(&definitions, &input)
}


pub fn parse(definitions: &BTreeMap<String, Token>, input: &str) -> Result<Token, ParserError> {
    // implicit outer parens
    // TODO w/o copy - modify iterator?
    let input = format!("({})", input);

    let mut iter = input.chars().peekable();

    parse_iter(definitions, &mut iter)
}

pub fn run(input: &str) -> Result<Token, Error> {
    let parsed_input = parse_file(input)?;

    println!("Parsed: {parsed_input} {parsed_input:#?}");

    let result = evaluate(parsed_input)?;

    Ok(result)
}
