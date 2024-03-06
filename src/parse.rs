use core::iter::Peekable;
use std::str::Chars;
use std::collections::BTreeMap;

use crate::ds::Token;
use crate::error::ParserError;
use crate::spec::RESERVED;

pub fn parse_applications(tokens: &mut Vec<Token>) -> Option<Token> {
    let argument = tokens.pop()?;

    parse_applications(tokens)
        .map(
            |predicate| Token::Application(Box::new([predicate, argument.clone()])
        ))
        .or(Some(argument))
}

pub fn parse_iter(definitions: &BTreeMap<String, Token>, iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {
    match iter.next() {
        // skip whitespace
        // hopefully the recursion gets optimized out
        Some(x) if x.is_whitespace() => parse_iter(definitions, iter),

        // open parentheses: build out token group
        Some('(') => {
            let mut tokens: Vec<Token> = Vec::new();
            let mut tokens = loop {
                match parse_iter(definitions, iter) {
                    Ok(token) => tokens.push(token),
                    // close the parentheses
                    Err(ParserError::ClosedParentheses) => break tokens,
                    // propagate other errors
                    Err(other_error) => Err(other_error)?,
                }
            };

            println!("tokens (open parentheses): {tokens:?}");

            parse_applications(&mut tokens).ok_or(ParserError::EmptyApplication)
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
            let mut name: String = x.into();

            while let Some(next_char) = iter.peek().filter(|c| !RESERVED.contains(**c)) {
                name.push(*next_char);
                iter.next();
            }

            if let Ok(num) = str::parse::<u32>(&name) {
                Token::Name(num)
            } else {
                definitions.get(&name)
                    .ok_or(ParserError::UndefinedMacroName { name })?
                    .clone()
            }
        }),

        // bad error - this means parser needed more, but input ended.
        // will propagate and be shown to end user.
        None => Err(ParserError::UnclosedParentheses),
    }
}
