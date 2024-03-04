use crate::ds::Token;

static OLD_TREES_SET_CAPACITY: usize = 10;

pub fn evaluate(tree: Token) -> Token {
    let mut old_trees: Vec<Token> = Vec::with_capacity(OLD_TREES_SET_CAPACITY);
    old_trees.push(tree);

    loop {
        let current_tree = &old_trees[old_trees.len() - 1];

        let new_tree = applied(current_tree);

        // detect non-beta-reducible functions
        // or lack of new applications to use
        // through naive hashset check to see if reduction is
        // not progressing.
        if old_trees.contains(&new_tree) {
            return new_tree;
        }

        old_trees.push(new_tree);

        print!("\n\n\n");
        println!("Tree step: {:#?}", old_trees);
    }
}

pub fn applied(application: &Token) -> Token {
    let Token::Application(tokens_box) = application else {
        return match application {
            Token::Function(token_box) => Token::Function(Box::new(applied(&*token_box))),
            x => x.clone(),
        };
    };

    let [ref function, ref argument] = **tokens_box;

    let function = if let Token::Application(_) = function {
        applied(&function)
    } else {
        function.clone()
    };

    let Token::Function(function_result) = replaced(&function, argument, 0) else {
        panic!("Called a function on non-function predicate {function:#?}");
    };

    *function_result
}

pub fn replaced(token: &Token, new: &Token, old: u32) -> Token {
    print!(
        "APPLYING REPLACEMENT OF NAME {} IN FUNCTION {:#?}",
        old, token
    );
    println!("  {}", token == &Token::Name(old));

    match token {
        Token::Name(_) if token == &Token::Name(old) => new.clone(),

        Token::Application(tokens_box) => {
            let [ref function, ref argument] = **tokens_box;

            Token::Application(Box::new([
                replaced(function, new, old + 1),
                replaced(argument, new, old + 1),
            ]))
        }

        Token::Function(token_box) => Token::Function(Box::new(replaced(token_box, new, old + 1))),

        x => x.clone(),
    }
}
