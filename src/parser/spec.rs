use crate::enums::Comparison;
use logos::{Lexer, Logos};

fn comment(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let txt = slice.split_at(1).1.trim(); // Trimming “#” and spaces
    Some(txt.to_string())
}

fn name(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_owned())
}

fn extra(lex: &mut Lexer<Token>) -> Vec<String> {
    let slice = lex.slice().trim();
    slice.split(',').map(|s| s.to_owned()).collect()
}

fn comparison(lex: &mut Lexer<Token>) -> Comparison {
    Comparison::get(lex.slice()).unwrap()
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z][a-zA-Z0-9._-]+", name, priority = 3)]
    Name(String),

    #[regex(r"\[[,a-zA-Z0-9._-]+\]", extra)]
    Extra(Vec<String>),

    #[regex(r"-e ")]
    Editable,

    #[regex(r"#(.*)", comment)]
    InlineComment(String),

    // #[regex(r"(<|<=)\d+", comparison, priority = 3)]
    // Operator(Comparison),
    #[regex(r"<", comparison)]
    LessThan(Comparison),

    #[regex(r"<=", comparison)]
    LessThanOrEqual(Comparison),

    #[regex(r"!=", comparison)]
    NotEqual(Comparison),

    #[regex(r"==", comparison)]
    Equal(Comparison),

    #[regex(r">=", comparison)]
    GreaterThanOrEqual(Comparison),

    #[regex(r">", comparison)]
    GreaterThan(Comparison),

    #[regex(r"~=", comparison)]
    CompatibleRelease(Comparison),

    #[regex(r"===", comparison)]
    ArbitraryEqual(Comparison),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
