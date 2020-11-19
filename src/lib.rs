use random_number::random;
use std::fmt::{Display};

pub enum Charsets {
    ASCII,
    Letters,
    LettersLowercase,
    LettersUppercase,
    Numbers,
    Special,
}

#[derive(Default)]
pub struct GenerationResult {
    result: Vec<char>
} impl GenerationResult {
    pub fn to_string(&self) -> String {
        self.result.iter().collect()
    }
} impl Display for GenerationResult {
    fn fmt (&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.to_string())
    }
}



pub struct Charset;
impl Charset {

    pub fn from_charsets(charsets: Charsets) -> Vec<char> {
        let data = match charsets {
            Charsets::ASCII => "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*()_+-={}[]:\";'\\|,.<>/?`~",
            Charsets::Letters => "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM",
            Charsets::LettersLowercase => "qwertyuiopasdfghjklzxcvbnm",
            Charsets::LettersUppercase => "QWERTYUIOPASDFGHJKLZXCVBNM",
            Charsets::Numbers => "1234567890",
            Charsets::Special => "!@#$%^&*()_+-={}[]:\";'\\|,.<>/?`~",
        };
        data.chars().into_iter().collect()
    }

    pub fn from_str(data: &str) -> Vec<char> {
        data.chars().into_iter().collect()
    }
}


pub struct RandomString;
impl RandomString {

    pub fn generate(length: i32, vec_charset: &Vec<char>) -> GenerationResult {

        if vec_charset.is_empty() {
            return Default::default();
        }

        let mut random_result: Vec<char> = Vec::new();
        let mut i: u8;
        for _ in 0..length {
            i = random!(..=(vec_charset.len() - 1) as u8);
            random_result.push(vec_charset[i as usize])
        }

        GenerationResult {
            result: random_result
        }
    }

}
