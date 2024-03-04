// λ

pub mod preprocessor;
pub mod spec;
pub mod macros;

pub mod application;
pub mod ds;
pub mod function;
pub mod numbers;
pub mod parens;
pub mod remove_group;
pub mod spaces;

pub mod run;

// TODO: Vec<Option<Token>> for less copying
//       then flatten at the end?

// TODO: error system improvement -- stop bubbling Err(())s
// (pest would do this automatically)

use ds::Token;

use preprocessor::preprocess;
use macros::parse_macros;

use application::parse_application;
use function::parse_functions;
use numbers::parse_numbers;
use parens::parse_parens;
use remove_group::parse_remove_group;
use spaces::parse_spaces;

use run::evaluate;

pub fn parse_file(input: &str) -> Result<Token, ()> {
    let (definitions, input) = preprocess(input)?;

    Ok(parse(&definitions, &input)?)
}

pub fn parse(definitions: &Vec<(String, Token)>, input: &str) -> Result<Token, ()> {

    // implicit outer parens
    // TODO w/o copy - modify iterator?
    let input = format!("({})", input);

    let mut iter = input.chars().peekable();

    // TODO refactor parse_parens to work on tokens like others
    let mut output = parse_parens(&mut iter).unwrap();
    parse_numbers(&mut output);
    parse_spaces(&mut output);
    parse_functions(&mut output)?;
    parse_application(&mut output)?;
    let output = parse_remove_group(output);
    let output = parse_macros(&definitions, &output);

    Ok(output)
}

pub fn run(input: &str) -> Result<Token, ()> {
    let parsed_input = parse_file(input)?;

    println!("Parsed: {:#?}", parsed_input);

    let result = evaluate(parsed_input);

    Ok(result)
}
