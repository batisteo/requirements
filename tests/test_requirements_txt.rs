use std::{fs::read_to_string, path::Path};

use requirements::{self, enums::Comparison, Requirement};

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
fn test_requirements_txt() {
    let path = Path::new("tests/requirements.txt");
    let file = read_to_string(&path).expect("Cannot read file");
    assert_eq!(
        requirements::parse_str(&file).unwrap(),
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
                line: String::from("aiohttp"),
                name: Some(String::from("aiohttp")),
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
                    (Comparison::LessThan, String::from("3"))
                ],
                ..Default::default()
            },
            Requirement {
                line: String::from("-e editable_package"),
                editable: false, // this is wrong
                ..Default::default()
            },
            Requirement {
                line: String::from("passlib[argon2,chacha]"),
                name: Some(String::from("passlib")),
                extras: vec![String::from("argon2"), String::from("chacha")],
                ..Default::default()
            }
        ]
    )
}
