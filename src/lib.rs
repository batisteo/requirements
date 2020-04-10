//! Fast parser of requirement files (.txt or .in)
//!
//! # Usage
//! __Soon…__

#[macro_use]
extern crate pest_derive;

mod parser;
pub mod enums;
pub mod requirements;
pub mod prelude;

use parser::parse;
pub use crate::requirements::Requirement;

/// Parses requirements from a string
pub fn parse_str(content: &str) -> Result<Vec<Requirement>, String> {
    parse(content)
}

// TODO: Add file loading code (recursive?) here.

