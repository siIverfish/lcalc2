use core::iter::Peekable;
use std::collections::BTreeMap;
use std::str::Chars;

use crate::ds::Token;
use crate::error::ParserError;
use crate::spec::RESERVED;

pub struct Parser<'a> {
    pub definitions: BTreeMap<String, Token>,
    pub iter: Peekable<Chars<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(definitions: BTreeMap<String, Token>, input: &'a str) -> Self {
        Parser{ definitions, iter: input.chars().peekable() }
    }
}

impl Parser<'_> {
    pub fn parse_applications(&mut self) -> Result<Token, ParserError> {
        let mut token = self.parse_iter()?;
    
        // keep consuming next tokens and adding them as arguments
        // left-associative function calling
        while let Ok(next_token) = self.parse_iter() {
            token = Token::Application(Box::new([token, next_token]))
        }
    
        Ok(token)
    }
    
    fn parse_character(&mut self, starting_character: char) -> Result<Token, ParserError> {
        let mut name: String = String::from(starting_character);
    
        while let Some(next_char) = self.iter.peek().filter(|c| !RESERVED.contains(**c)) {
            name.push(*next_char);
            self.iter.next();
        }
    
        Ok(if let Ok(num) = str::parse::<u32>(&name) {
            Token::Name(num)
        } else {
            self.definitions
                .get(&name)
                .ok_or(ParserError::UndefinedMacroName { name })?
                .clone()
        })
    }
    
    pub fn parse_iter(&mut self) -> Result<Token, ParserError> {
        match self.iter.next() {
            // open parentheses: build out token group
            Some('(') => self.parse_applications(),
            // close an open token group
            Some(')') => Err(ParserError::ClosedParentheses)?,
            // just make a function with everything after it
            Some('λ') => Ok(Token::Function(Box::new(self.parse_iter()?))),
            // skip whitespace
            // hopefully the recursion gets optimized out
            Some(c) if c.is_whitespace() => self.parse_iter(),
            // construct MacroName / Name out of character
            Some(c) => self.parse_character(c),
            // bad error - this means parser needed more, but input ended.
            // will propagate and be shown to end user.
            None => Err(ParserError::UnclosedParentheses),
        }
    }
}


