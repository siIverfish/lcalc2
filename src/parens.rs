use core::iter::Peekable;
use std::str::Chars;

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
       },
       Some(')') => Err(()),
       Some(x ) => Ok(Token::Char(x)),
       None => Err(()), // ig running out of space = closeparens
   }
}
