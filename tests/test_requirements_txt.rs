#![allow(unused, dead_code)]
use requirements::{self, enums::Comparison, Requirement};
use std::fs;

#[test]
fn test_one_requirement() {
    assert_eq!(
        requirements::parse_str(&"pip\n").unwrap(),
        vec![Requirement {
            line: String::from("pip"),
            name: Some(String::from("pip")),
            ..Default::default()
        }]
    )
}
#[test]
fn test_editable_requirement() {
    assert_eq!(
        requirements::parse_str(&"\n-e django\n").unwrap(),
        vec![Requirement {
            line: String::from("-e django"),
            name: Some(String::from("django")),
            editable: true,
            ..Default::default()
        }]
    )
}

#[test]
fn print() {
    let file = fs::read_to_string("tests/requirements.txt").expect("Cannot read file");
    let mut list = requirements::parse_str(&file).unwrap().into_iter();
    for req in list {
        println!("{}", req)
    }
}
fn test_requirements_txt() {
    let file = fs::read_to_string("tests/requirements.txt").expect("Cannot read file");
    let list = requirements::parse_str(&file).unwrap();
    println!("{:?}", &list);
    let mut iter = list.into_iter();
    assert_eq!(
        iter.next().unwrap(),
        Requirement {
            line: String::from("aiohttp"),
            name: Some(String::from("aiohttp")),
            ..Default::default()
        }
    );
    vec![
        Requirement {
            line: String::from("# Example requirement file"),
            ..Default::default()
        },
        Requirement {
            line: String::from("--extra-index-url https://pypi.example.com/pypi"),
            ..Default::default()
        },
        Requirement {
            line: String::from("black==19.10b0"),
            name: Some(String::from("black")),
            specs: vec![(Comparison::Equal, String::from("19.10b0"))],
            ..Default::default()
        },
        Requirement {
            line: String::from("chardet  # upgrade at will"),
            name: Some(String::from("chardet")),
            comment: Some(String::from("upgrade at will")),
            ..Default::default()
        },
        Requirement {
            line: String::from("Django>=2,<3"),
            name: Some(String::from("Django")),
            specs: vec![
                (Comparison::GreaterThanOrEqual, String::from("2")),
                (Comparison::LessThan, String::from("3")),
            ],
            ..Default::default()
        },
        Requirement {
            line: String::from("-e editable_package"),
            editable: true,
            ..Default::default()
        },
        Requirement {
            line: String::from("passlib[argon2,chacha]"),
            name: Some(String::from("passlib")),
            extras: vec![String::from("argon2"), String::from("chacha")],
            ..Default::default()
        },
    ];
}
