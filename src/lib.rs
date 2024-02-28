
pub mod parens;
pub mod ds;
pub mod function;
pub mod numbers;
pub mod spaces;
pub mod application;
pub mod remove_group;

// TODO: Vec<Option<Token>> for less copying
//       then flatten at the end?

use ds::Token;

use application ::parse_application;
use function    ::parse_functions;
use parens      ::parse_parens;
use numbers     ::parse_numbers;
use remove_group::parse_remove_group;
use spaces      ::parse_spaces;

pub fn parse(input: &str) -> Result<Token, ()> {
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
    let output = parse_remove_group(output)?;

    Ok(output)
}