use crate::*;


/// Generates a random string with given length and charset
///
/// # Arguments
///
/// * `length` - Length of a string to generate
/// * `charset` - Charset obiect with a valid body
///
/// # Examples
///
/// ```
/// use random_string::generate;
///
/// println!("{}", generate(6, &charset));
/// ```

pub fn generate(length: usize, charset: &str) -> String {
    let body = charset;
    let result = String::with_capacity(length);
    
    for _ in 0..length {
      body.chars().nth(fastrand::usize(0..charset.len()));
    }

    result
}
