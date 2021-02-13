pub use crate::enums::{Comparison, VersionControlSystem};
use std::fmt;
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
            extras: vec![],
            specs: vec![],
            extra_index_url: String::new(),
        }
    }
}

impl fmt::Display for Requirement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default = Self::default();
        let mut body: Vec<String> = vec![];
        if self.name != default.name {
            body.push(format!("name: {}", self.name.clone().unwrap()))
        };
        if self.editable {
            body.push(format!("editable: {}", self.editable))
        };
        if self.comment != default.comment {
            body.push(format!("comment: {}", self.comment.clone().unwrap()))
        };
        if !self.specifier {
            body.push(format!("specifier: {}", self.specifier))
        };
        if self.subdirectory != default.subdirectory {
            body.push(format!(
                "subdirectory: {}",
                self.subdirectory.clone().unwrap()
            ))
        };
        if self.hash_name != default.hash_name {
            body.push(format!("hash_name: {}", self.hash_name.clone().unwrap()))
        };
        if self.hash != default.hash {
            body.push(format!("hash: {}", self.hash.clone().unwrap()))
        };
        if self.revision != default.revision {
            body.push(format!("revision: {}", self.revision.clone().unwrap()))
        };
        if self.extras != default.extras {
            body.push(format!("{:?}", self.extras.clone()))
        };
        if self.extra_index_url != default.extra_index_url {
            body.push(self.extra_index_url.clone())
        };
        write!(
            f,
            "Requirement {{ {} }} {}",
            body.join(", "),
            self.line.clone()
        )
    }
}
