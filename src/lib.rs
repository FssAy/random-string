use random_number::random;
use std::cmp::min;

pub enum Charset {
    ASCII,
    Letters,
    LettersLowercase,
    LettersUppercase,
    Numbers,
    Special,
}

pub struct RandomString;

impl RandomString {

    fn split_by_length(data: String, length: usize) -> Result<Vec<String>, &'static str> {
        if data == "".to_string() {
            return Err("String data can't be empty")
        }

        let mut vector_result: Vec<String> = vec![];
        let mut str_temp: &str = data.as_str();

        while !str_temp.is_empty() {
            let (pref, suf) = str_temp.split_at(min(length, str_temp.len()));
            vector_result.push(pref.to_string());
            str_temp = suf;
        }

        Ok(vector_result)
    }

    pub fn get_charset(charset: Charset) -> String {
        return match charset {
            Charset::ASCII => "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*()_+-={}[]:\";'\\|,.<>/?`~".to_string(),
            Charset::Letters => "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM".to_string(),
            Charset::LettersLowercase => "qwertyuiopasdfghjklzxcvbnm".to_string(),
            Charset::LettersUppercase => "QWERTYUIOPASDFGHJKLZXCVBNM".to_string(),
            Charset::Numbers => "1234567890".to_string(),
            Charset::Special => "!@#$%^&*()_+-={}[]:\";'\\|,.<>/?`~".to_string(),
        }
    }

    pub fn generate(length: i32, charset: String) -> Result<String, &'static str>{
        let vec_charset;
        match RandomString::split_by_length(charset, 1) {
            Ok(result) => vec_charset = result,
            Err(msg) => return Err(msg)
        }

        let mut random_string: String = "".to_string();
        let mut i: u8;
        for _ in 0..length {
            i = random!(..=(vec_charset.len() - 1) as u8);
            random_string += vec_charset[i as usize].as_str();
        }

        Ok(random_string)
    }

}
