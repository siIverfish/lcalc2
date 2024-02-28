

use crate::ds::Token;

fn parse_application_tokens(tokens: &mut Vec<Token>) -> Result<(), ()> {
    Ok(if tokens.len() == 1 {
        // tokens.len() is 1, so there must be an element at index 0.
        parse_application(tokens.get_mut(0).unwrap())?;
    } 
    else {
        // clones value, todo fix
        let mut first_token = tokens.get(0).ok_or(())?.clone();
        parse_application(&mut first_token)?;
    
        let mut next_tokens = tokens.split_off(1);
        parse_application_tokens(&mut next_tokens)?;
        let next_tokens = Token::Group(next_tokens);

        let application = Token::Application(
            Box::new((first_token, next_tokens))
        );

        tokens.clear();
        tokens.push(application);
    })
}

pub fn parse_application(token: &mut Token) -> Result<(), ()> {
    Ok(match token {
        Token::Group(tokens) => {
            parse_application_tokens(tokens)?;
        }
        Token::Function(token_box) => {
            let Token::Group(tokens) = &mut **token_box else { unreachable!() };
            parse_application_tokens(tokens)?;
        }
        _ => {},
    })
}