//! Fast parser of requirement files (txt or in)
//!
//! # Usage
//! __Soon…__

#[macro_use]
extern crate pest_derive;

mod enums;

use enums::{Comparison, VersionControlSystem};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::path::Path;

impl VersionControlSystem {
    pub fn _from_rule(rule: Rule) -> VersionControlSystem {
        use VersionControlSystem::*;
        match rule {
            Rule::git => Git,
            Rule::hg => Mercurial,
            Rule::svn => Subversion,
            Rule::bzr => Bazaar,
            _ => Unknown,
        }
    }
}

#[derive(Parser)]
#[grammar = "requirements.pest"]
pub struct RequirementParser;

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
    fn new<'a>() -> Requirement<'a> {
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

fn parse_version(pair: Pair<Rule>) -> (Comparison, String) {
    let mut version = pair.into_inner();
    let comparison = Comparison::from_rule(version.next().unwrap().as_rule());
    let number = String::from(version.next().unwrap().as_str());
    (comparison, number)
}

fn parse_package(parsed: Pair<'_, Rule>) -> Requirement<'_> {
    let mut package = Requirement::new();
    for line in parsed.into_inner() {
        package.line = line.as_str().into();
        for pair in line.into_inner() {
            match &pair.as_rule() {
                Rule::name => package.name = Some(pair.as_str().into()),
                Rule::version => package.specs.push(parse_version(pair)),
                Rule::extras => {
                    package.extras = pair
                        .into_inner()
                        .map(|extra| extra.as_str().into())
                        .collect()
                }
                Rule::comment => {
                    package.comment = {
                        let comment = pair.into_inner().next().unwrap().as_str();
                        if comment.is_empty() {
                            None
                        } else {
                            Some(comment.into())
                        }
                    }
                }
                Rule::extra_index_url => package.extra_index_url = pair.as_str().into(),
                _ => (),
            }
        }
    }
    package
}

pub fn parse(unparsed_file: &str) -> Result<Vec<Requirement>, Error<Rule>> {
    let requirement_file = RequirementParser::parse(Rule::requirement_file, &unparsed_file)?
        .next()
        .unwrap();

    // copy_of_requirement_file = requirement_file.copy();
    let requirements: Vec<Requirement> = requirement_file
        .into_inner()
        .filter(|pair| pair.as_rule() == Rule::line)
        .map(parse_package)
        .collect();

    // let metas: Vec<Requirement> = copy_of_requirement_file
    //     .into_inner()
    //     .filter(|pair| pair.as_rule() == Rule::meta)
    //     .map(parse_package)
    //     .collect();
    // println!("{:?}", metas);
    Ok(requirements)
}
