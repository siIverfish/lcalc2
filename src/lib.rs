// λ

#![feature(box_patterns)]

pub mod error;
pub mod macros;
pub mod preprocessor;
pub mod spec;

pub mod ds;
pub mod parens;

pub mod run;

// TODO: HashMap or BTreeMap?

use std::{collections::BTreeMap, iter::Peekable, str::Chars};

use ds::Token;
use error::{Error, ParserError};
use macros::parse_macros;
use preprocessor::preprocess;

use parens::parse_parens;

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

pub fn parse_iter(definitions: &BTreeMap<String, Token>, iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {

    // TODO refactor parse_parens to work on tokens like others
    let output = parse_parens(definitions, iter)?;
    let output = parse_macros(definitions, output)?;

    Ok(output)
}

pub fn run(input: &str) -> Result<Token, Error> {
    let parsed_input = parse_file(input)?;

    println!("Parsed: {parsed_input} {parsed_input:#?}");

    let result = evaluate(parsed_input)?;

    Ok(result)
}
