use core::iter::Peekable;
use std::str::Chars;

use crate::error::ParserError;
use crate::ds::Token;
use crate::spec::RESERVED;

pub fn parse_parens(iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {
    Ok(match iter.next() {
        Some('(') => {
            let mut tokens: Vec<Token> = Vec::new();
            loop {
                match parse_parens(iter) {
                    Ok(token) => tokens.push(token),
                    Err(ParserError::ClosedParentheses) => return Ok(Token::Group(tokens)),
                    Err(other_error) => Err(other_error)?,
                }
            }
        }
        Some(')') => Err(ParserError::ClosedParentheses)?,
        Some('λ') => Token::LambdaSymbol,
        Some(' ') => Token::Char(' '),
        Some(x) => {
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
        }
        None => Err(ParserError::UnclosedParentheses)?
    })
}
