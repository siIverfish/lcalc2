use crate::ds::Token;

pub fn parse_remove_group(token: Token) -> Token {
    match token {
        Token::Group(tokens) => parse_remove_group(tokens.into_iter().next().unwrap()),
        Token::Function(token_box) => Token::Function(Box::new(parse_remove_group(*token_box))),
        Token::Application(tokens_box) => {
            let [function, argument] = *tokens_box;

            Token::Application(Box::new([
                parse_remove_group(function),
                parse_remove_group(argument),
            ]))
        }
        x => x,
    }
}
