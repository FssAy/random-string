# random-string
A simple crate that allows you to generate random strings based on a given charset and length.

## Usage
Add a dependence in `Cargo.toml`
```toml
[dependencies]
random-string = "1.0"
``` 

## Example
```rust
use random_string::generate;

fn main() {
    let charset = "1234567890";

    println!("[{}]", generate(6, charset));
}
```
