pub trait ToCharset {
    fn to_chars(&self) -> Vec<char>;
}

impl ToCharset for String {
    fn to_chars(&self) -> Vec<char> {
        self.chars().into_iter().collect()
    }
}

impl ToCharset for &str {
    fn to_chars(&self) -> Vec<char> {
        self.chars().into_iter().collect()
    }
}


/// A set of specific chars to use in generation
#[derive(Debug)]
pub struct Charset {
    length: usize,
    body: Vec<char>,
}

impl Charset {
    /// Returns a charset with a given body as &str or String
    /// Will return None only if the given body is empty
    ///
    /// # Arguments
    ///
    /// * `charset` - Body of the Charset
    ///
    /// # Examples
    ///
    /// ```
    /// use random_string::Charset;
    /// let charset = Charset::new("name");
    /// ```
    pub fn new<T>(charset: T) -> Option<Self>
    where T: ToCharset {
        let chars = charset.to_chars();
        let len = chars.len();

        // let mut buf = BytesMut::with_capacity(chars.len());
        // buf.put(chars.as_bytes());

        if len == 0 {
            return None
        }

        Some( Self {
            length: len,
            body: chars,
        })
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn borrow_body(&self) -> &Vec<char> {
        &self.body
    }

    pub fn clone_body(&self) -> Vec<char> {
        self.body.clone()
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for ch in self.body.iter() {
            result.push(*ch)
        }
        result
    }
}
