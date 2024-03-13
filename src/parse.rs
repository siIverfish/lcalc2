use core::iter::Peekable;
use std::collections::BTreeMap;

use crate::ds::{FlatToken, Token};
use crate::error::ParserError;

type ParserItem = Result<Token, ParserError>;

pub struct Parser<'a> {
    pub definitions: BTreeMap<String, Token>,
    pub iter: Peekable<Box<dyn Iterator<Item = FlatToken> + 'a>>
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser{ 
            definitions: BTreeMap::new(), 
            iter: (
                Box::new(std::iter::empty::<FlatToken>()) 
                as Box<dyn Iterator<Item = FlatToken>>
            ).peekable()
        }
    }

    pub fn with_definition_and_input(definitions: BTreeMap<String, Token>, input: Vec<FlatToken>) -> Self {
        Parser{ 
            definitions,
            iter: (
                Box::new(input.into_iter()) 
                as Box<dyn Iterator<Item = FlatToken>>
            ).peekable()
        }
    }

    pub fn add_input<'b: 'a>(mut self, input: Vec<FlatToken>) -> Self {
        let new_iter = self.iter.chain(input.into_iter());
        let boxed = Box::new(new_iter) as Box<dyn Iterator<Item = FlatToken>>;

        self.iter = boxed.peekable();
        self
    }
}

impl Parser<'_> {
    pub fn parse_applications(&mut self, consume_closeparen: bool) -> ParserItem {
        let mut token = self.next_iter()?;
    
        // keep consuming next tokens and adding them as arguments
        // left-associative function calling
        while let Ok(next_token) = self.next_iter() {
            dbg!(self.iter.peek());
            dbg!(&next_token);
            token = Token::Application(Box::new([token, next_token]));
            if !consume_closeparen && self.iter.peek() == Some(&FlatToken::CloseParen) {
                break;
            }
        }
    
        Ok(token)
    }

    // fn parse_character(&mut self, starting_character: char) -> Result<Token, ParserError> {
    //     let mut name: String = String::from(starting_character);
    
    //     while let Some(next_char) = self.iter.peek().filter(|c| !RESERVED.contains(**c)) {
    //         name.push(*next_char);
    //         self.iter.next();
    //     }
    
    //     Ok(if let Ok(num) = str::parse::<u32>(&name) {
    //         Token::Name(num)
    //     } else {
    //         self.definitions
    //             .get(&name)
    //             .ok_or(ParserError::UndefinedMacroName { name })?
    //             .clone()
    //     })
    // }

    fn next_iter(&mut self) -> ParserItem {
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

        match self.iter.next().ok_or(ParserError::ClosedParentheses)? {
            // open parentheses: build out token group
            FlatToken::OpenParen => self.parse_applications(true),
            // close an open token group
            FlatToken::CloseParen => Err(ParserError::ClosedParentheses),
            // just make a function with everything after it
            FlatToken::Lambda => {
                self.iter.next();
                Ok(
                    Token::Function(
                        Box::new(
                            self.parse_applications(false)?
                        )
                    )
                )
            },
            FlatToken::MacroName(name) => self.definitions
                .get(&name)
                .cloned()
                .ok_or(ParserError::UndefinedMacroName { name }),
            FlatToken::Name(number) => Ok(Token::Name(number)),
        }
    }
}


