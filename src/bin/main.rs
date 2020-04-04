use globwalk::GlobError;
use requirements::{self, Requirement};
use std::{env, fs, path::Path};
use walkdir::DirEntry;

fn find_req_files() -> Result<Vec<DirEntry>, GlobError> {
    let globwalker = globwalk::GlobWalkerBuilder::from_patterns(".", &["*req*.{txt,in}"])
        .max_depth(4)
        .follow_links(true)
        .build()?
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    Ok(globwalker)
}

fn parse_requirements(path: &Path) -> () {
    let content = fs::read_to_string(&path).expect("Cannot read file");
    let requirement_file: Vec<Requirement> = requirements::parse(&content).unwrap();
    for requirement in requirement_file.into_iter() {
        println!("{:?}", requirement.line);
    }
}

fn show_parsing(args: &Vec<String>) -> () {
    if args.len() > 1 {
        parse_requirements(Path::new(&args[1]))
    } else {
        for path in find_req_files().unwrap() {
            parse_requirements(&path.path())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    show_parsing(&args);
}
