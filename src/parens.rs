use core::iter::Peekable;
use std::str::Chars;

use crate::spec::RESERVED;
use crate::ds::Token;

pub fn parse_parens(iter: &mut Peekable<Chars>) -> Result<Token, ()> {
    match iter.next() {
        Some('(') => {
            let mut tokens: Vec<Token> = Vec::new();
            while let Ok(token) = parse_parens(iter) {
                tokens.push(token);
            }

            // there must have just been an error returned
            // the only error case is a closeparen
            // so the group is closed
            Ok(Token::Group(tokens))
        }
        Some(')') => Err(()),
        Some('λ') => Ok(Token::LambdaSymbol),
        Some(x) => Ok({
            let mut token: String = x.into();

            while let Some(next_char) = iter.peek().filter(|c| !RESERVED.contains(**c)) {
                println!("next char getting...");
                token.push(*next_char);
                iter.next();
            }

            if let Ok(num) = str::parse::<u32>(&token) {
                Token::Name(num)
            } else {
                Token::MacroName(token)
            }
        }),
        None => Err(()), // automatically close expression -- is this functionality good?
    }
}
