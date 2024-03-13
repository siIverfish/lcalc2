// λ

#![feature(box_patterns)]

pub mod error;
pub mod preprocessor;
pub mod spec;

pub mod ds;
pub mod parse;
pub mod preparser;

pub mod run;

// TODO: HashMap or BTreeMap?
// TODO: Peekable::next_if

use std::path::Path;

use ds::Token;
use error::{Error, ParserError};
use preprocessor::preprocess;

use parse::Parser;

use run::evaluate;

pub use preparser::preparse;

pub fn parse_file(path: &Path) -> Result<Token, ParserError> {
    let (definitions, input) = preprocess(path)?;

    let input = preparse(&input);

    let mut parser = Parser::with_definition_and_input(definitions, input);

    parser.parse_applications(true)
}

pub fn run(path: &Path) -> Result<Token, Error> {
    let parsed_input = parse_file(path)?;

    println!("Parsed: {parsed_input} {parsed_input:#?}");

    let result = evaluate(parsed_input)?;

    Ok(result)
}
