use pest::iterators::Pair;
use pest::Parser;
use crate::enums::{VersionControlSystem, Comparison};
use crate::requirements::Requirement;

impl VersionControlSystem {
    pub fn from_rule(rule: Rule) -> VersionControlSystem {
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

impl Comparison {
    pub fn from_rule(rule: Rule) -> Comparison {
        use Comparison::*;
        match rule {
            Rule::less_than => LessThan,
            Rule::less_than_or_equal => LessThanOrEqual,
            Rule::not_equal => NotEqual,
            Rule::equal => Equal,
            Rule::greater_than_or_equal => GreaterThanOrEqual,
            Rule::greater_than => GreaterThan,
            Rule::compatible_release => CompatibleRelease,
            Rule::arbitrary_equal => ArbitraryEqual,
            _ => ArbitraryEqual,
        }
    }
}

#[derive(Parser)]
#[grammar = "parser/requirements.pest"]
pub struct RequirementParser;

fn parse_version(pair: Pair<Rule>) -> (Comparison, String) {
    let mut version = pair.into_inner();
    let comparison = Comparison::from_rule(version.next().unwrap().as_rule());
    let number = String::from(version.next().unwrap().as_str());
    (comparison, number)
}

fn parse_package(parsed: Pair<'_, Rule>) -> Requirement<'_> {
    let mut package = Requirement::new();
    for line in parsed.into_inner() {
        package.line = String::from(line.as_str()).clone();
        for pair in line.into_inner() {
            match &pair.as_rule() {
                Rule::name => package.name = Some(String::from(pair.as_str()).clone()),
                Rule::version => package.specs.push(parse_version(pair)),
                Rule::extras => {
                    package.extras = pair
                        .into_inner()
                        .map(|extra| String::from(extra.as_str()).clone())
                        .collect()
                }
                Rule::comment => {
                    package.comment = {
                        let comment = pair.into_inner().next().unwrap().as_str();
                        if comment.is_empty() {
                            None
                        } else {
                            Some(String::from(comment).clone())
                        }
                    }
                }
                Rule::extra_index_url => package.extra_index_url = String::from(pair.as_str()).clone(),
                _ => (),
            }
        }
    }
    package
}

pub fn parse(unparsed_file: &str) -> Result<Vec<Requirement>, String> {
    let req_file = match RequirementParser::parse(Rule::requirement_file, unparsed_file) {
        Ok(mut rules) => rules.next().unwrap(),
        Err(_) => return Err(String::from("Failed to parse requirements"))
    };

    let requirements: Vec<Requirement> = req_file
        .into_inner()
        .filter(|pair| pair.as_rule() == Rule::line)
        .map(parse_package)
        .collect();

    Ok(requirements)
}

