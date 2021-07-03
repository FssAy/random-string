/// Generates a random string with given length and charset.
///
/// Will Panic if the provided charset is empty.
///
/// # Arguments
///
/// * `length` - Length of a `String` to generate
/// * `charset` - Object implementing `AsRef` trait to do reference-to-reference conversion into `&str`
///
/// # Example
///
/// ```
/// use random_string::generate;
///
/// let charset = "1234567890";
/// println!("{}", generate(6, charset));
/// ```

pub fn generate<S: AsRef<str>>(length: usize, charset: S) -> String {
    let charset_str = charset.as_ref();

    if charset_str.is_empty() {
        panic!("Provided charset is empty! It should contain at least one character");
    }

    let chars: Vec<char> = charset_str.chars().collect();
    let mut result = String::with_capacity(length);

    unsafe {
        for _ in 0..length {
            result.push(
                *chars.get_unchecked(fastrand::usize(0..chars.len()))
            );
        }
    }

    result
}
