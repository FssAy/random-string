# random-string
A simple crate that allows you to generate random strings based on a given charset and length.

## "Benchmark"
Generating 3 times 100000 results with length 10 and Letters charset.

version 0.1.1
```
Benchmark: 3.3298s
Benchmark: 3.3292s
Benchmark: 3.3292s
```

version 0.1.2
```
Benchmark: 0.659s
Benchmark: 0.634s
Benchmark: 0.635s
```

Benchmark test src
```rust
fn execute() {
    use crate::{RandomString, Charset, Charsets};
    use chrono::prelude::*;

    let time_start = Utc::now();

    let charset = Charset::from_charsets(Charsets::Letters);
    for _ in 0..100000 {
        let _ = RandomString::generate(10, &charset);
    }

    let time_end = Utc::now() - time_start;
    println!("Benchmark: {}.{}s", time_end.num_seconds(), time_end.num_milliseconds());
}

fn main() {
    for _ in 0..3 {
        execute();
    }
}
```

## Usage
Add a dependence in `Cargo.toml`
```toml
[dependencies]
random-string = "0.1.2"
``` 

## Functions

**`Charset::from_charsets(Charsets)` -> `Vec<char>`**<br> 
Get a ready charset by passing an enum *`Charsets`*. <br>
The output is a Vector containing certain characters that will be used in the generation.<br>

**`Charset::from_str(&str)` -> `Vec<char>`** <br> 
Create a charset by passing an *`&str`*. <br>
The output is a Vector containing certain characters that will be used in the generation.<br>


**`RandomString::generate(i32, &Vec<char>)` -> `GenerationResult`** <br> Generate a random Vector containing characters with a certain length as *`i32`* and based on a *`&Vec<char>`* charset. 
The output is a *`GenerationResult`*. 

**`GenerationResult.to_string()` -> `String`** <br> 
Convert the *`GenerationResult`* into a *`String`*.

## Enums
Use this enum in `RandomString::get_charset` function to get a charset.
```rust
enum Charsets {
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
use random_string::{RandomString, Charset, Charsets};

fn main() {
    
    let charset = Charset::from_charsets(Charsets::Numbers);
    // or
    let charset = Charset::from_str("1234567890");
    
    let data = RandomString::generate(6, &charset);
    println!("Generated: {}", data);

    let data_string = RandomString::generate(6, &charset).to_string();
    println!("Generated: {}", data_string);

}
```
