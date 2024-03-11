use core::iter::Peekable;
use std::collections::BTreeMap;
use std::path::Iter;

use crate::ds::Token;
use crate::error::ParserError;
use crate::spec::RESERVED;

type ParserItem = Result<Token, ParserError>

pub struct Parser<'a> {
    pub definitions: BTreeMap<String, Token>,
    pub iter: Peekable<Box<dyn Iterator<Item = char> + 'a>>
}

pub struct ApplicationParser<'a> {
    parser: Parser<'a>,
}

impl Iterator for ApplicationParser<'_> {
    type Item = ParserItem;

    fn next(&mut self) -> Option<ParserItem> {
        let mut token = match self.parser.next()? {
            Ok(token) => token,
            Err(e) => return Some(Err(e))
        };
    
        // keep consuming next tokens and adding them as arguments
        // left-associative function calling
        while let Some(Ok(next_token)) = self.parser.next() {
            dbg!(&next_token);
            token = Token::Application(Box::new([token, next_token]))
        }
    
        Some(Ok(token))
    }
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser{ 
            definitions: BTreeMap::new(), 
            iter: (
                Box::new(std::iter::empty::<char>()) 
                as Box<dyn Iterator<Item = char>>
            ).peekable()
        }
    }

    pub fn with_definition_and_input(definitions: BTreeMap<String, Token>, input: &'a str) -> Self {
        Parser{ 
            definitions,
            iter: (
                Box::new(input.chars()) 
                as Box<dyn Iterator<Item = char>>
            ).peekable()
        }
    }

    pub fn add_input<'b: 'a>(mut self, input: &'b str) -> Self {
        let new_iter = self.iter.chain(input.chars());
        let boxed = Box::new(new_iter) as Box<dyn Iterator<Item = char>>;

        self.iter = boxed.peekable();
        self
    }
}

impl Parser<'_> {
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

}

impl Iterator for Parser<'_> {
    type Item = ParserItem;

    fn next(&mut self) -> Option<ParserItem> {
        // self.iter.next().and_then(|character| {
        //     Some(match character {
        //         // open parentheses: build out token group
        //         '(' => return ApplicationParser{ parser: self }.next(),
        //         // close an open token group
        //         ')' => Err(ParserError::ClosedParentheses),
        //         // just make a function with everything after it
        //         'λ' => Ok(
        //             Token::Function(
        //                 Box::new(
        //                     match (ApplicationParser { parser: self }.next()?) {
        //                         Ok(t) => t,
        //                         Err(e) => return Some(Err(e))
        //                     }
        //                 )
        //             )
        //         ),
        //         // skip whitespace
        //         // hopefully the recursion gets optimized out
        //         c if c.is_whitespace() => return self.next(),
        //         // construct MacroName / Name out of character
        //         c => self.parse_character(c),
        //         // bad error - this means parser needed more, but input ended.
        //         // will propagate and be shown to end user.
        //     })
        // }).or(Some(Err(ParserError::ClosedParentheses)))

        Some(match match self.iter.next() {
                Some(c) => c,
                None => return Some(Err(ParserError::ClosedParentheses))
            } {
            // open parentheses: build out token group
            '(' => return ApplicationParser { parser: self }.next(),
            // close an open token group
            ')' => Err(ParserError::ClosedParentheses),
            // just make a function with everything after it
            'λ' => Ok(
                Token::Function(
                    Box::new(
                        match (ApplicationParser { parser: self }.next()?) {
                            Ok(t) => t,
                            Err(e) => return Some(Err(e))
                        }
                    )
                )
            ),
            // skip whitespace
            // hopefully the recursion gets optimized out
            c if c.is_whitespace() => return self.next(),
            // construct MacroName / Name out of character
            c => self.parse_character(c),
            // bad error - this means parser needed more, but input ended.
            // will propagate and be shown to end user.
        })
    }
}


