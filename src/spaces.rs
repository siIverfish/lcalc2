use crate::ds::Token;

pub fn parse_spaces(token: &mut Token) {
    match token {
        Token::Group(tokens) => {
            // remove all Token::Char(' ') from the group
            // leaving adjacent names unmerged
            tokens.retain(|token| {
                if let Token::Char(c) = token {
                    !c.is_whitespace()
                } else {
                    true
                }
            });

            // also remove spaces from subtokens
            for token in tokens {
                parse_spaces(token);
            }
        }
        // do nothing to other tokens.
        // spaces are removed via their vec.
        _ => {},
    }
}
