use std::hash::{DefaultHasher, Hash, Hasher};

use crate::ds::Token;
use crate::error::EvaluationError;
use crate::spec::MAX_RECURSION_DEPTH;

pub fn hash_token(token: &Token) -> u64 {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    hasher.finish()
}

pub fn evaluate(mut tree: Token) -> Result<Token, EvaluationError> {
    let mut old_trees_hashes: Vec<u64> = Vec::new();

    for _ in 0..MAX_RECURSION_DEPTH {
        tree = evaluate_once(tree)?;
        let new_hash = hash_token(&tree);

        if old_trees_hashes.contains(&new_hash) {
            return Ok(tree);
        }

        old_trees_hashes.push(new_hash);
    }

    Err(EvaluationError::RecursionDepth { tree })
}

pub fn evaluate_once(tree: Token) -> Result<Token, EvaluationError> {
    match tree {
        Token::Application(tokens_box) => {
            let [function, argument] = *tokens_box;

            // evaluate function to prepare it for the call
            // e.g. in call of function (λ1 λ1) on argument X
            // the function must be evaluated to produce a comprehensible value.
            let function = evaluate(function)?;

            // substitute the argument into the function
            // e.g (λλλ3) X := (λλλX)
            // e.g. (λ1) X := λX
            // e.g. λ(λ2(1 1))(λ2(1 1))X := λ(λX(1 1))(λX(1 1))
            let function = replace(function, 0, &argument);

            // remove now-redundant outer function
            // now that its been replaced.
            // if there isn't an outer function, something has gone badly wrong.
            if let Token::Function(box new_expression) = function {
                Ok(new_expression)
            } else {
                Err(EvaluationError::NonFunctionPredicate { 
                    predicate: function, 
                    argument: argument 
                })
            }
        }
        other => Ok(other),
    }
}

pub fn replace(tree: Token, old_name: u32, new_token: &Token) -> Token {
    match tree {
        Token::Name(name) if name == old_name => new_token.clone(),
        Token::Function(box token_box) => {
            Token::Function(Box::new(replace(token_box, old_name + 1, new_token)))
        }
        other => other.recurse_with(|token| replace(token, old_name, new_token)),
    }
}

// --------------------------------------------------------------------------------------
// |                     the following is old, unused code.                             |
// |                     it will be removed in the future.                              |
// --------------------------------------------------------------------------------------

// pub fn evaluate(tree: Token) -> Token {
//     let mut old_trees: Vec<Token> = Vec::with_capacity(10);
//     old_trees.push(tree);

//     loop {
//         let current_tree = &old_trees[old_trees.len() - 1];

//         let new_tree = applied(current_tree);

//         // detect non-beta-reducible functions
//         // or lack of new applications to use
//         // through naive hashset check to see if reduction is
//         // not progressing.
//         if old_trees.contains(&new_tree) {
//             return new_tree;
//         }

//         println!("Newly reduced tree: {new_tree}");

//         old_trees.push(new_tree);

//         print!("\n\n\n");
//     }
// }

// pub fn applied(application: &Token) -> Token {
//     let Token::Application(tokens_box) = application else {
//         return match application {
//             // don't eagerly simplify insides of functions
//             // remove when sure
//             Token::Function(token_box) => Token::Function(Box::new(evaluate((**token_box).clone()))),
//             x => x.clone(),
//         };
//     };

//     let [ref function, ref argument] = **tokens_box;

//     let function = if let Token::Application(_) = function {
//         println!("Applying function {function}");
//         applied(&function)
//     } else {
//         function.clone()
//     };

//     if let Token::Function(function_result) = evaluate(replaced(&function, argument, 0)) {
//         *function_result
//     } else {
//         application.clone()
//         // eager simplification ??
//         // Token::Application(Box::new([
//         //     evaluate(function),
//         //     argument.clone()
//         // ]))
//         // panic!("Called function on non-function predicate {function}");
//     }
// }

// pub fn replaced(token: &Token, new: &Token, old: u32) -> Token {
//     // print!(
//     //     "APPLYING REPLACEMENT OF NAME {} IN FUNCTION {:#?}",
//     //     old, token
//     // );
//     // println!("  {}", token == &Token::Name(old));

//     match token {
//         Token::Name(_) if token == &Token::Name(old) => new.clone(),

//         Token::Application(tokens_box) => {
//             let [ref function, ref argument] = **tokens_box;

//             Token::Application(Box::new([
//                 replaced(function, new, old),
//                 replaced(argument, new, old),
//             ]))
//         }

//         Token::Function(token_box) => Token::Function(Box::new(replaced(token_box, new, old + 1))),

//         x => x.clone(),
//     }
// }
