#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Token {
    Group(Vec<Token>),
    Char(char),
    MacroName(String),
    LambdaSymbol,
    Name(u32),
    Function(Box<Token>),
    Application(Box<[Token; 2]>),
}
