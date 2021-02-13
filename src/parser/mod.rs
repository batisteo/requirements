mod spec;

use crate::enums::LineType;
use crate::requirements::Requirement;
use logos::{Lexer, Logos};
use spec::Token as t;

fn comment<'a>(lex: &mut Lexer<'a, Line<'a>>) -> Option<String> {
    let slice = lex.slice();
    let txt = slice.split_at(1).1.trim(); // Trimming “#” and spaces
    Some(txt.to_string())
}

fn spec<'a>(lex: &mut Lexer<'a, Line<'a>>) -> Option<Requirement<'a>> {
    let mut requirement = Requirement::default();
    let line = lex.slice();
    requirement.line = String::from(line);
    for token in t::lexer(line) {
        match token {
            t::Name(name) => requirement.name = Some(name),
            t::Editable => requirement.editable = true,
            _ => (),
        }
    }
    Some(requirement)
}

fn extra_index_url<'a>(lex: &mut Lexer<'a, Line<'a>>) -> Option<String> {
    let line = lex.slice().trim();
    Some(String::from(line[18..].trim()))
}

#[derive(Logos, Debug, PartialEq)]
pub enum Line<'a> {
    #[regex("#.*", comment)]
    Comment(String),

    #[regex(r"[a-zA-Z0-9-._]+.*", spec)]
    Spec(Requirement<'a>),

    #[regex(r"--extra-index-url https?://.*", extra_index_url)]
    ExtraIndexUrl(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl<'a> Line<'a> {
    pub fn to_line_type(self) -> Option<LineType<'a>> {
        match self {
            Line::Comment(comment) => Some(LineType::Comment(comment)),
            Line::Spec(spec) => Some(LineType::Specification(spec)),
            Line::ExtraIndexUrl(url) => Some(LineType::ExtraIndexUrl(url)),
            Line::Error => None,
        }
    }
}

pub fn parse(unparsed_file: &str) -> Vec<Requirement> {
    unparsed_file
        .lines()
        .map(|line| Line::lexer(line))
        .filter_map(|mut lex| lex.next())
        .filter_map(|parsed_line| parsed_line.to_line_type())
        .filter_map(|line_type| match line_type {
            LineType::Specification(req) => Some(req),
            _ => None,
        })
        .collect()
}
