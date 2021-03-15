# random-string
A simple crate that allows you to generate random strings based on a given charset and length.

## Usage
Add a dependence in `Cargo.toml`
```toml
[dependencies]
random-string = "0.2.0"
``` 

## Example
```rust
use random_string::{
    Charset, 
    GenResult, 
    generate,
};


fn main() {
    let charset_str = Charset::new("1234567890").unwrap();

    let body = String::from("1234567890");
    let charset_string = Charset::new(&*body).unwrap();

    print_result(
        generate(6, &charset_str)
    );

    print_result(
        generate(6, &charset_string)
    );
}

fn print_result(result: GenResult) {
    println!("Generated String: {}", result);
    println!("Generated Chars: {:?}", result);
}
```
