use crate::ds::Token;

pub fn parse_numbers(tree: &mut Token) {
    // ONLY TAKES "Token::Group"s

    match tree {
        Token::Group(tokens) => {
            let mut i = 0;

            while i <= tokens.len() {
                if let Some(Token::Char(character)) = tokens.get(i) {
                    if let Some(current_name) = character.to_digit(10) {
                        if let Some(Token::Name(prev_name)) =
                            (i).checked_sub(1).map(|j| tokens.get(j)).flatten()
                        {
                            tokens[i] = Token::Name(prev_name * 10 + current_name);

                            tokens.remove(i - 1);
                            i -= 1; // accounts for removed token
                        } else {
                            tokens[i] = Token::Name(current_name);
                        }
                    }
                } else {
                    tokens.get_mut(i).map(parse_numbers);
                }

                i += 1;
            }
        }
        // a better implementation would have a solid state machine
        // in the Token enum that accounted for all possibilities
        _ => {}
    }
}
