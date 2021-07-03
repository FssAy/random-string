# random-string
A simple crate that allows you to generate random strings based on a given charset and length.

## Usage
Add a dependence in `Cargo.toml`
```toml
[dependencies]
random-string = "0.2"
``` 

## Example
```rust
use random_string::generate;

fn main() {
    let charset = "1234567890";

    print_result(
        generate(6, charset)
    );

    print_result(
        generate(6, charset)
    );
}

fn print_result(result: GenResult) {
    println!("Generated String: {}", result);
    println!("Generated Chars: {:?}", result);
}
```
