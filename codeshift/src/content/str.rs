use crate::content;
use crate::content::Peek;

pub struct Str(String);

impl Peek for Str {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        if content.starts_with(&self.0) {
            Ok((&content[..self.0.len()], &content[self.0.len()..]))
        } else {
            let found = content::found(content, &self.0);
            Err(format!("Expected '{}' but found '{}'", self.0, found))
        }
    }
}

pub fn str(s: &str) -> Str {
    Str(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;
        #[test]
        fn short() {
            let input = "123abcd456";
            let (output, rest) = str("123").peek(input).unwrap();
            assert_eq!(output, "123");
            assert_eq!(rest, "abcd456");
        }

        #[test]
        fn long() {
            let input = "123abcd456";
            let (output, rest) = str("123abcd456").peek(input).unwrap();
            assert_eq!(output, "123abcd456");
            assert_eq!(rest, "");
        }

        #[test]
        fn not_exists() {
            let input = "123abcd456";
            let result = str("xyz").peek(input);
            assert_eq!(
                result,
                Err("Expected 'xyz' but found '123abcd456'".to_string())
            );
        }

        #[test]
        fn not_exists_at_start() {
            let input = "123abcd456";
            let result = str("1234").peek(input);
            assert_eq!(
                result,
                Err("Expected '1234' but found '123abcd456'".to_string())
            );
        }
    }
}
