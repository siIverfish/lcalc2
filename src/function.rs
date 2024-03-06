use crate::error::ParserError;

use crate::ds::Token;

pub fn parse_functions(tree: &mut Token) -> Result<(), ParserError> {
    if let Token::Group(tokens) = tree {
        // recurse to interior groups
        for token in &mut *tokens {
            if let Token::Group(_) = token {
                parse_functions(token)?;
            }
        }

        let Some(index) = tokens
            .iter()
            .enumerate()
            .find(|(_, n)| n == &&Token::LambdaSymbol)
            .map(|(i, _)| i)
        else {
            // no lambda expression in group, do nothing
            return Ok(());
        };

        // found a function!

        // return Err(()) if there's nothing after after the function
        // e.g. λ1(λ)
        //          ^
        tokens
            .get(index + 1)
            .ok_or(ParserError::MalformedFunction)?;

        let next_tokens = tokens.split_off(index + 1);
        let mut next_tokens_group = Token::Group(next_tokens);
        parse_functions(&mut next_tokens_group)?;

        let function_token = Token::Function(Box::new(next_tokens_group));

        tokens[index] = function_token;
    }

    Ok(())
}
