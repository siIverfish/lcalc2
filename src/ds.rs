

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
   Group(Vec<Token>),
   Char(char),
   Name(u32),
   Function(Box<Token>),
   Application(Box<(Token, Token)>),
}
