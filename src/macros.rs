
use crate::ds::Token;

pub fn parse_macros(definitions: &Vec<(String, Token)>, token: &Token) -> Token  {
    match token {
        Token::MacroName(name) => definitions
            .iter()
            .filter(|(k, _)| k == name)
            .next()
            .expect("macro not defined")
            .1
            .clone(),

        // unimportant recursion boilerplate.
        Token::Application(tokens_box) => {
            let [ref function, ref argument] = **tokens_box;

            Token::Application(Box::new([
                parse_macros(definitions, function),
                parse_macros(definitions, argument),
            ]))
        }
        Token::Function(token_box) => Token::Function(Box::new(parse_macros(definitions, token_box))),
        other => other.clone(),
    }
}