//! Fast parser of requirement files (.txt or .in)
//!
//! `requirements` is a fast and convenient way to parse Python's requirements file. It (will)
//! supports recursively parsing requirements files and resolving and identifying version
//! version constraints to satisfy all dependencies.
//!
//! # Usage
//!
//! Below is a basic example of how to use this library:
//!
//! ```
//! fn main() {
//!     let content = "Django>=3.0.0";
//!     let reqs = requirements::parse_str(&content).unwrap();
//!
//!     for req in reqs.into_iter() {
//!         println!("{:?}", req);
//!     }
//! }
//! ```

#[macro_use]
extern crate pest_derive;

mod parser;
pub mod enums;
pub mod requirements;

use parser::parse;
pub use crate::requirements::Requirement;

/// Parses requirements from a string
pub fn parse_str(content: &str) -> Result<Vec<Requirement>, String> {
    parse(content)
}

// TODO: Add file loading code (recursive?) here.

