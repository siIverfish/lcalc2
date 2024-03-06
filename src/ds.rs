use crate::error::ParserError;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum Token {
    Group(Vec<Token>),
    Char(char),
    MacroName(String),
    LambdaSymbol,
    Name(u32),
    Function(Box<Token>),
    Application(Box<[Token; 2]>),
}

use std::fmt::Display;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Application(tokens_box) => {
                write!(f, "({} {})", (*tokens_box)[0], (*tokens_box)[1])
            }
            Token::Function(token_box) => write!(f, "λ{}", *token_box),
            Token::Name(num) => write!(f, "{num}"),
            _ => write!(f, "<other: {self:#?}"),
        }
    }
}

impl Token {
    pub fn recurse_with_result(
        self,
        f: impl Fn(Token) -> Result<Token, ParserError>,
    ) -> Result<Self, ParserError> {
        Ok(match self {
            Token::Application(tokens_box) => {
                let [function, argument] = *tokens_box;
                Token::Application(Box::new([f(function)?, f(argument)?]))
            }
            Token::Function(token_box) => Token::Function(Box::new(f(*token_box)?)),
            // Application and Function are the only recursive enums.
            other => other,
        })
    }

    pub fn recurse_with(self, f: impl Fn(Self) -> Self) -> Self {
        match self {
            Token::Application(tokens_box) => {
                let [function, argument] = *tokens_box;
                Token::Application(Box::new([f(function), f(argument)]))
            }
            Token::Function(token_box) => Token::Function(Box::new(f(*token_box))),
            // Application and Function are the only recursive enums.
            other => other,
        }
    }
}
