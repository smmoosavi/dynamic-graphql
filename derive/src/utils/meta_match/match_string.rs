pub trait MatchString: Sized {
    fn match_string(str: &str) -> Option<darling::Result<Self>>;
}

impl MatchString for String {
    fn match_string(string: &str) -> Option<darling::Result<Self>> {
        Some(Ok(string.to_string()))
    }
}
