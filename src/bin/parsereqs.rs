use globwalk::GlobError;
use requirements::{self, prelude::*};
use std::{env, fs, path::Path};
use walkdir::DirEntry;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        parse_requirements(Path::new(&args[1]))
    } else {
        for path in find_requirement_files().unwrap() {
            parse_requirements(&path.path())
        }
    }
}

fn parse_requirements(path: &Path) -> () {
    let content = fs::read_to_string(&path).expect("Cannot read file");
    let requirement_file: Vec<Requirement> = requirements::parse_str(&content).unwrap();

    for requirement in requirement_file.into_iter() {
        if let Some(name) = requirement.name {
            print!("{} ", name);
        }
        if let Some(comment) = requirement.comment {
            print!("({}) ", comment)
        }
        for (comparison, version) in requirement.specs.into_iter() {
            print!("{}{} ", comparison, version);
        }
        println!();
    }
}

fn find_requirement_files() -> Result<Vec<DirEntry>, GlobError> {
    // TODO: Use requirements directly for dependency globbing and resolution
    let globwalker = globwalk::GlobWalkerBuilder::from_patterns(".", &["*req*.{txt,in}"])
        .max_depth(4)
        .follow_links(true)
        .build()?
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    Ok(globwalker)
}
