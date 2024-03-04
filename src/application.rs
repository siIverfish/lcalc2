use crate::ds::Token;

fn parse_application_tokens(tokens: &mut Vec<Token>) -> Result<(), ()> {
    // TODO improve this function

    Ok(if tokens.len() == 1 {
        // tokens.len() is 1, so there must be an element at index 0.
        parse_application(tokens.get_mut(0).unwrap())?;
    } else {
        // clones value, todo fix

        let mut first_token = tokens.remove(0);
        parse_application(&mut first_token)?;

        let mut second_token = tokens.remove(0);
        parse_application(&mut second_token)?;

        let application = Token::Application(Box::new([first_token, second_token]));

        tokens.insert(0, application);

        parse_application_tokens(tokens)?;
    })
}

pub fn parse_application(token: &mut Token) -> Result<(), ()> {
    Ok(match token {
        Token::Group(tokens) => {
            parse_application_tokens(tokens)?;
        }
        Token::Function(token_box) => {
            let Token::Group(tokens) = &mut **token_box else {
                unreachable!()
            };
            parse_application_tokens(tokens)?;
        }
        _ => {}
    })
}
