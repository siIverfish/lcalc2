// λ

#![feature(box_patterns)]

pub mod error;
pub mod macros;
pub mod preprocessor;
pub mod spec;

pub mod application;
pub mod ds;
pub mod function;
pub mod parens;
pub mod remove_group;

pub mod run;

// TODO: Vec<Option<Token>> for less copying
//       then flatten at the end?

// TODO: error system improvement -- stop bubbling Err(())s
// (pest would do this automatically)

// TODO: HashMap or BTreeMap?

use std::collections::BTreeMap;

use ds::Token;
use error::{Error, ParserError};
use macros::parse_macros;
use preprocessor::preprocess;

use application::parse_application;
use function::parse_functions;
use parens::parse_parens;
use remove_group::parse_remove_group;

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

    // TODO refactor parse_parens to work on tokens like others
    let mut output = parse_parens(&mut iter)?;
    parse_functions(&mut output)?;
    parse_application(&mut output);
    let output = parse_remove_group(output);
    let output = parse_macros(definitions, output)?;

    Ok(output)
}

pub fn run(input: &str) -> Result<Token, Error> {
    let parsed_input = parse_file(input)?;

    println!("Parsed: {parsed_input} {parsed_input:#?}");

    let result = evaluate(parsed_input)?;

    Ok(result)
}
