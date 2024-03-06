// λ

#![feature(box_patterns)]

pub mod error;
pub mod preprocessor;
pub mod spec;

pub mod ds;
pub mod parse;

pub mod run;

// TODO: HashMap or BTreeMap?

use ds::Token;
use error::{Error, ParserError};
use preprocessor::preprocess;

use parse::Parser;

use run::evaluate;

pub fn parse_file(input: &str) -> Result<Token, ParserError> {
    let (definitions, input) = preprocess(input)?;

    let mut parser = Parser { definitions, iter: input.chars().peekable() };

    parser.parse_applications()
}

pub fn run(input: &str) -> Result<Token, Error> {
    let parsed_input = parse_file(input)?;

    println!("Parsed: {parsed_input} {parsed_input:#?}");

    let result = evaluate(parsed_input)?;

    Ok(result)
}
