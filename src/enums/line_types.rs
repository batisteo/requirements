use crate::Requirement;

/// Different supported type per line
#[derive(Debug, PartialEq)]
pub enum LineType<'a> {
    Specification(Requirement<'a>),
    Comment(String),
    ExtraIndexUrl(String),
    Editable,
}
