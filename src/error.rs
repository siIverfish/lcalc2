use thiserror::Error;

use crate::ds::Token;
use crate::spec::MAX_RECURSION_DEPTH;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("input ended, but open parentheses were still unclosed.")]
    UnclosedParentheses,

    #[error("input ongoing, but final end-paren occurred.")]
    ClosedParentheses,

    #[error("malformed function")]
    MalformedFunction,

    #[error("undefined macro name: {name}")]
    UndefinedMacroName { name: String },
}

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("application to non-function predicate {predicate} with argument {argument}")]
    NonFunctionPredicate { predicate: Token, argument: Token },
    #[error("pasing {tree:?} recursed more than maximum of {MAX_RECURSION_DEPTH}")]
    RecursionDepth { tree: Token },
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("parser error: {0}")]
    Parse(#[from] ParserError),
    #[error("evaluation error: {0}")]
    Evaluation(#[from] EvaluationError),
}
