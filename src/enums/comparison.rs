use crate::Rule;
use std::fmt;

#[derive(Debug)]
pub enum Comparison {
    LessThan,
    LessThanOrEqual,
    NotEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    CompatibleRelease,
    ArbitraryEqual,
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

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Comparison::*;
        let sign = match self {
            LessThan => "<",
            LessThanOrEqual => "<=",
            NotEqual => "!=",
            Equal => "==",
            GreaterThanOrEqual => ">=",
            GreaterThan => ">",
            CompatibleRelease => "~=",
            ArbitraryEqual => "===",
        };
        write!(f, "{}", sign)
    }
}
