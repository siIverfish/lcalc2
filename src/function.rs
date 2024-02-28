

use crate::ds::Token;

pub fn parse_functions(tree: &mut Token) -> Result<(), ()> {
    match tree {
        Token::Group(tokens) => {
            // recurse to interior groups
            for token in &mut *tokens {
                if let Token::Group(_) = token {
                    parse_functions(token)?;
                }
            }

            let Some(index) = tokens
                .iter()
                .enumerate()
                .filter(|(_, n)| n == &&Token::Char('λ'))
                .next()
                .map(|(i, _)| i)
            else {
                // no lambda expression in group, do nothing
                return Ok(()); 
            };

            // found a function!
            
            // return Err(()) if there's nothing after after the function
            // e.g. λ1(λ)
            //          ^ 
            tokens.get(index+1).ok_or(())?;

            let next_tokens = tokens.split_off(index+1);
            let mut next_tokens_group = Token::Group(next_tokens);
            parse_functions(&mut next_tokens_group)?;

            let function_token = Token::Function(
                Box::new(
                    next_tokens_group
                )
            );

            tokens[index] = function_token;
        },
        _ => {},
    }

    Ok(())
}