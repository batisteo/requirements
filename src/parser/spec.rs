use logos::{Lexer, Logos};

fn comment(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let txt = slice.split_at(1).1.trim(); // Trimming “#” and spaces
    Some(txt.to_string())
}

fn name(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_owned())
}

fn string(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_owned()
}

fn extra(lex: &mut Lexer<Token>) -> Vec<String> {
    let slice = lex.slice().trim();
    slice.split(',').map(|s| s.to_owned()).collect()
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z0-9._-]+", name)]
    Name(String),

    #[regex(r"\[[,a-zA-Z0-9._-]+\]", extra)]
    Extra(Vec<String>),

    #[regex(r"#(.*)", comment)]
    InlineComment(String),

    #[regex(r"<", string)]
    LessThan(String),

    #[regex(r"<=", string)]
    LessThanOrEqual(String),

    #[regex(r"!=", string)]
    NotEqual(String),

    #[regex(r"==", string)]
    Equal(String),

    #[regex(r">=", string)]
    GreaterThanOrEqual(String),

    #[regex(r">", string)]
    GreaterThan(String),

    #[regex(r"~=", string)]
    CompatibleRelease(String),

    #[regex(r"===", string)]
    ArbitraryEqual(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

enum Operator {
    LessThan,
    LessThanOrEqual,
    NotEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    CompatibleRelease,
    ArbitraryEqual,
}

impl Operator {
    fn key(operator: &str) -> Option<Operator> {
        use Operator::*;
        match operator {
            "<" => Some(LessThan),
            "<=" => Some(LessThanOrEqual),
            "!=" => Some(NotEqual),
            "==" => Some(Equal),
            ">=" => Some(GreaterThanOrEqual),
            ">" => Some(GreaterThan),
            "~=" => Some(CompatibleRelease),
            "===" => Some(ArbitraryEqual),
            &_ => None,
        }
    }

    fn value(&self) -> &str {
        use Operator::*;
        match self {
            LessThan => "<",
            LessThanOrEqual => "<=",
            NotEqual => "!=",
            Equal => "==",
            GreaterThanOrEqual => ">=",
            GreaterThan => ">",
            CompatibleRelease => "~=",
            ArbitraryEqual => "===",
        }
    }
}
