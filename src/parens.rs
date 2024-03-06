use core::iter::Peekable;
use std::str::Chars;
use std::collections::BTreeMap;

use crate::ds::Token;
use crate::error::ParserError;
use crate::spec::RESERVED;

use crate::parse_iter;

pub fn parse_parens(definitions: &BTreeMap<String, Token>, iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {
    match iter.next() {
        // skip whitespace
        // hopefully the recursion gets optimized out
        Some(x) if x.is_whitespace() => parse_parens(definitions, iter),

        // open parentheses: build out token group
        Some('(') => {
            let mut tokens: Vec<Token> = Vec::new();
            loop {
                match parse_parens(definitions, iter) {
                    Ok(token) => tokens.push(token),
                    // close the parentheses
                    Err(ParserError::ClosedParentheses) => return Ok(Token::Group(tokens)),
                    // propagate other errors
                    Err(other_error) => Err(other_error)?,
                }
            }
        }

        // close an open token group
        Some(')') => Err(ParserError::ClosedParentheses)?,

        // create a function, using the next tokens.
        Some('λ') => Ok(
            Token::Function(
                Box::new(
                    parse_iter(definitions, iter)?
                )
            )
        ),

        // construct MacroName / Name out of character
        Some(x) => Ok({
            let mut token: String = x.into();

            while let Some(next_char) = iter.peek().filter(|c| !RESERVED.contains(**c)) {
                token.push(*next_char);
                iter.next();
            }

            if let Ok(num) = str::parse::<u32>(&token) {
                Token::Name(num)
            } else {
                Token::MacroName(token)
            }
        }),

        // bad error - this means parser needed more, but input ended.
        // will propagate and be shown to end user.
        None => Err(ParserError::UnclosedParentheses),
    }
}
