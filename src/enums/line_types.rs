use crate::parser::Line;
use LineType::*;

/// Different supported type per line
#[derive(Debug, PartialEq)]
pub enum LineType {
    Specification(String),
    Comment(String),
    ExtraIndexUrl(String),
    Editable,
}
