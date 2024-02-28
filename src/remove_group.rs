
use crate::ds::Token;

pub fn parse_remove_group(token: Token) -> Result<Token, ()> {
    Ok(match token {
        Token::Group(tokens) => parse_remove_group(
            tokens
                .into_iter()
                .next()
                .unwrap()
        )?,
        Token::Function(token_box) => Token::Function(
            Box::new(
                parse_remove_group(*token_box)?
            )
        ),
        Token::Application(tokens_box) => Token::Application(
            Box::new(
                (
                    parse_remove_group((*tokens_box).0)?, 
                    parse_remove_group((*tokens_box).1)?
                )
            )
        ),
        Token::Name(num) => Token::Name(num),
        
        _ => Err(())?,
    })
}