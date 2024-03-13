

use crate::ds::FlatToken;
use crate::spec::RESERVED;


pub fn preparse(input: &str) -> Vec<FlatToken> {
    let mut tokens: Vec<FlatToken> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() 
    {
        match c {
            '(' => Some(FlatToken::OpenParen),
            ')' => Some(FlatToken::CloseParen),
            'λ' => Some(FlatToken::Lambda),
            c if c.is_whitespace() => None,
            c => {
                let mut string: String = String::from(c);

                while let Some(next_char) = iter.next_if(|&c| !RESERVED.contains(c))
                {
                    string.push(next_char);
                }

                Some(if let Ok(number) = str::parse::<u32>(&string) {
                    FlatToken::Name(number)
                } else {
                    FlatToken::MacroName(string)
                })
            }
        }.map(|token| tokens.push(token));
    }
    

    dbg!(&tokens);

    // implicit parentheses around function body
    let mut index = 0;

    while let Some(token) = tokens.get(index) {
        if token == &FlatToken::Lambda {
            // TODO panics here
            tokens.insert(index+1, FlatToken::OpenParen);

            let mut parentheses_depth = 0;
            let mut j_index = 1;
            while let Some(next_token) = tokens.get(index+j_index) {
                parentheses_depth += 
                    match next_token {
                        &FlatToken::CloseParen => -1,
                        &FlatToken::OpenParen => 1,
                        _other => 0
                    };

                if parentheses_depth == 0 {
                    tokens.insert(index+j_index, FlatToken::CloseParen);
                    break;
                }
                
                j_index += 1;
            }

            // if tokens.get(index+j_index).is_none() {
            //     // TODO
            //     panic!("failed implicit parens");
            // }
        }
        index += 1;
    }

    dbg!(&tokens);
    
    //application parentheses
    
    // let mut parentheses_depth = 0;
    // let mut index = 0;
    // while let Some(next_token) = tokens.get(index) {
    //     parentheses_depth += 
    //         match next_token {
    //             &FlatToken::CloseParen => -1,
    //             &FlatToken::OpenParen => 1,
    //             _other => {
    //                 tokens.insert(0, FlatToken::OpenParen);
    //                 0
    //             }
    //         };

    //     if parentheses_depth == 0 {
    //         tokens.insert(index, FlatToken::CloseParen);
    //         break;
    //     }
        
    //     index += 1;
    // }

    tokens
}