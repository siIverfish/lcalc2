use std::collections::BTreeMap;

use crate::ds::Token;
use crate::error::ParserError;

pub fn parse_macros(
    definitions: &BTreeMap<String, Token>,
    token: Token,
) -> Result<Token, ParserError> {
    Ok(match token {
        Token::MacroName(name) => match definitions.get(&name) {
            Some(token) => token.clone(),
            None => Err(ParserError::UndefinedMacroName { name })?,
        },
        other => other.recurse_with_result(|token| parse_macros(definitions, token))?,
    })
}
