# requirements

Fast parser of requirement files (txt or in)

## Usage

Example:

```rust
use requirements;

fn main() {
    let content = "Django>=3.0.0";
    let reqs = requirements::parse_str(&content).unwrap();

    for req in reqs.into_iter() {
        println!("{:?}", req);
    }
}

```

## License

This project is Free Software and available under the MIT license. See
the `LICENSE` file for more details.

