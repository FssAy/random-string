# random-string
A simple crate that allows you to generate random strings based on a given charset and length.

## Description
This is just a test crate for my personal use, and I don't think that there would be more updates. <br>
If you would like to change or add something, or even create a better version feel free to do so. 

## Usage
Add a dependence in `Cargo.toml`
```toml
[dependencies]
random-string = "0.1.1"
``` 

## Functions

`RandomString::get_charset(Charset)` <br> 
Get a ready charset by passing an enum `Charset`. <br>
The output is a String containing certain characters that will be used in the generation.<br>
<br>
`RandomString::generate(i32, String)` - Generate a random string with a certain length as `i32` and based on a `string` charset. 
The output is a Result with the generated String if succeeded, or an error message (static str) if failed. 

*Every charset is a String containing characters that you would like to use in generation process.
Remember that it can't be empty.*

## Enums
Use this enum in `RandomString::get_charset` function to get a charset.
```rust
enum Charset {
    ASCII,              // ASCII characters
    Letters,            // All lowercase and uppercase letters
    LettersLowercase,   // Lowercase letters from a to z
    LettersUppercase,   // Uppercase letters from A to Z
    Numbers,            // Numbers from 0 to 9
    Special             // All the special characters I could find on my keyboard
}
```

## Examples
```rust
use random_string::{RandomString, Charset};

fn main() {
    
    let charset = RandomString::get_charset(Charset::Numbers);
    // or
    let charset = "1234567890".to_string();
    
    match RandomString::generate(6, charset) {
        Ok(string) => println!("{}", string),
        Err(msg) => println!("Error: {}", msg)
    }
}
```
