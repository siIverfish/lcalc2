use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::ds::Token;
use crate::error::ParserError;
use crate::parse::Parser;
use crate::preparse;
use crate::spec::{COMMENT_SYMBOL, MACRO_DELIMITER};

// will use for multiline macro defininitions
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

// TODO: different datatype for root_path
pub fn preprocess(path: &Path) -> Result<(BTreeMap<String, Token>, String), ParserError> {
    println!("Processing file {}", path.display());

    let input: &str = &std::fs::read_to_string(path)?;

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
    let mut parser = Parser::new();

    while let Some((name, value)) = lines_iter
        .peek()
        .and_then(|line| line.split_once(MACRO_DELIMITER))
        .map(|(n, v)| (n.trim(), v.trim()))
    {
        if let Some(module_name) = name.strip_prefix('(').and_then(|name| name.strip_suffix(')')) {
            // this is a module declaration
            // this should be defined
            let module_path = path.with_file_name(value.to_owned() + ".lc");
            let (module_definitions, _) = preprocess(&module_path.clone())?;

            for (other_name, other_value) in module_definitions.into_iter() {
                // don't import macros imported in that file
                // just new ones
                if name.contains('.') {continue};
                let other_name = 
                    if module_name.is_empty() {
                        other_name
                    } else {
                        module_name.to_owned() + "." + &other_name
                    };

                parser.definitions.insert(other_name, other_value);
            }
        } else {
            // this is a variable declaration 
            let value = preparse(value);
            parser = parser.add_input(value);
            let value = parser.parse_applications(true)?;
            parser.definitions.insert(name.to_owned(), value);
        }
        
        lines_iter.next(); // silly
    }

    // now, the remaining lines must be the "expression" of the file.
    let expression: String = lines_iter.collect();

    dbg!(&expression);
    println!("{expression:#?}");

    Ok((parser.definitions, expression))
}
