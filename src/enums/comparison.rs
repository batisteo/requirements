use std::fmt;

/// Semver comparison operators
///
/// Abstraction of semver comparison operators.
#[derive(Debug, PartialEq)]
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

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Comparison {
    pub fn get(comparison: &str) -> Option<Self> {
        use Comparison::*;
        match comparison {
            "<" => Some(LessThan),
            "<=" => Some(LessThanOrEqual),
            "!=" => Some(NotEqual),
            "==" => Some(Equal),
            ">=" => Some(GreaterThanOrEqual),
            ">" => Some(GreaterThan),
            "~=" => Some(CompatibleRelease),
            "===" => Some(ArbitraryEqual),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        use Comparison::*;
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
