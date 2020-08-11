//! Fast parser of Python's requirement files
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
//! let content = "Django>=3.0.5";
//! let reqs = requirements::parse_str(&content).unwrap();
//!
//! for req in reqs.into_iter() {
//!     println!("{:?}", req);
//! }
//! ```

#[macro_use]
extern crate pest_derive;

pub mod enums;
mod parser;
mod requirements;

pub use crate::requirements::Requirement;
pub use parser::parse;

/// Parses requirements from a string
pub fn parse_str(content: &str) -> Result<Vec<Requirement>, String> {
    Ok(parse(content).map(Iterator::collect)?)
}

// TODO: Add file loading code (recursive?) here.
