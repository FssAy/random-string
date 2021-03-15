use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Debug, Result};
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
/// use random_string::Charset;
///
/// let charset = Charset::new("123");
/// println!("{}", generate(6, &charset));
/// ```
pub fn generate(length: usize, charset: &Charset) -> GenResult {
    let mut result = VecDeque::<char>::new();
    let body = charset.borrow_body();

    for _ in 0..length {
        result.push_back(
            *body.get(fastrand::usize(0..charset.length())).unwrap()
        )
    }

    GenResult {
        body: result
    }
}


/// Generation result holding a vector of chars
pub struct GenResult {
    body: VecDeque<char>,
}

impl GenResult {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for ch in self.body.iter() {
            result.push(*ch)
        }
        result
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn borrow_body(&self) -> &VecDeque<char> {
        &self.body
    }

    pub fn borrow_body_mut(&mut self) -> &mut VecDeque<char> {
        &mut self.body
    }

    pub fn clone_body(&self) -> VecDeque<char> {
        self.body.clone()
    }
}

impl Display for GenResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.to_string(),
        )
    }
}

impl Debug for GenResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?}",
            self.borrow_body(),
        )
    }
}
