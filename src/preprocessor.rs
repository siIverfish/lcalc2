use std::collections::BTreeMap;

use crate::ds::Token;
use crate::error::ParserError;
use crate::parse::Parser;
use crate::spec::{COMMENT_SYMBOL, MACRO_DELIMITER};

fn validate_parentheses(input: &str) -> bool {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum::<i32>()
        == 0
}

// TODO: validate names (no spaces, lambdas, can't start with number)

pub fn preprocess(input: &str) -> Result<(BTreeMap<String, Token>, String), ParserError> {
    // remove comments & empty lines, leaving only ':=' statements
    // and the expression
    let mut lines_iter = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.starts_with(COMMENT_SYMBOL))
        .filter(|s| !s.is_empty())
        .peekable();

    // process comments into a mapping
    // e.g. "one := λλ2 1" -> {"one": "λλ2 1"}
    let mut definitions: BTreeMap<String, Token> = BTreeMap::new();

    while let Some((name, value)) = lines_iter
        .peek()
        .and_then(|line| line.split_once(MACRO_DELIMITER))
        .map(|(n, v)| (n.trim(), v.trim()))
    {
        // this is a temporary solution. definitions will not be cloned every iteration in
        // the future, i just have to lay down some code for mutability in
        // the Parser struct.
        let value = Parser::new(definitions.clone(), &value).parse_applications()?;
        definitions.insert(name.to_owned(), value);
        lines_iter.next(); // silly
    }

    // now, the remaining lines must be the "expression" of the file.
    let expression: String = lines_iter.collect();

    Ok((definitions, expression))
}
