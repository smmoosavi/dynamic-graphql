use crate::content::{len, Peek};

pub struct After(String);

impl Peek for After {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        let index = content.find(&self.0).unwrap_or(content.len()) + self.0.len();
        len(index).peek(content)
    }
}

pub fn after(s: &str) -> After {
    After(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;

        #[test]
        fn start() {
            let input = "123abcd456";
            let (output, rest) = after("123").peek(input).unwrap();
            assert_eq!(output, "123");
            assert_eq!(rest, "abcd456");
        }

        #[test]
        fn short() {
            let input = "123abcd456";
            let (output, rest) = after("abcd").peek(input).unwrap();
            assert_eq!(output, "123abcd");
            assert_eq!(rest, "456");
        }

        #[test]
        fn not_exists() {
            let input = "123abcd456";
            let (output, rest) = after("xyz").peek(input).unwrap();
            assert_eq!(output, "123abcd456");
            assert_eq!(rest, "");
        }
    }
}
