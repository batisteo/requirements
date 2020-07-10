pub use crate::enums::{Comparison, VersionControlSystem};
use std::path::Path;


#[derive(Debug, PartialEq)]
pub struct Requirement<'a> {
    pub line: String,
    pub editable: bool,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub specifier: bool,
    pub vcs: Option<VersionControlSystem>,
    pub uri: Option<&'a Path>,
    pub subdirectory: Option<String>,
    pub path: Option<&'a Path>,
    pub hash_name: Option<String>,
    pub hash: Option<String>,
    pub revision: Option<String>,
    pub extras: Vec<String>,
    pub specs: Vec<(Comparison, String)>,
    pub extra_index_url: String,
}

impl Requirement<'_> {
    pub fn new<'a>() -> Requirement<'a> {
        Requirement::default()
    }
}

impl Default for Requirement<'_> {
    fn default() -> Self {
        Requirement {
            line: String::new(),
            name: None,
            specs: vec![],
            extras: vec![],
            comment: None,
            specifier: true,
            editable: false,
            uri: None,
            subdirectory: None,
            path: None,
            hash_name: None,
            hash: None,
            revision: None,
            vcs: None,
            extra_index_url: String::new(),
        }
    }
}