use crate::content::{len, Peek};

pub struct Until(String);

impl Peek for Until {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        let index = content.find(&self.0).unwrap_or(content.len());
        len(index).peek(content)
    }
}

pub fn until(s: &str) -> Until {
    Until(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;

        #[test]
        fn start() {
            let input = "123abcd456";
            let (output, rest) = until("123").peek(input).unwrap();
            assert_eq!(output, "");
            assert_eq!(rest, "123abcd456");
        }

        #[test]
        fn short() {
            let input = "123abcd456";
            let (output, rest) = until("abcd").peek(input).unwrap();
            assert_eq!(output, "123");
            assert_eq!(rest, "abcd456");
        }

        #[test]
        fn not_exists() {
            let input = "123abcd456";
            let (output, rest) = until("xyz").peek(input).unwrap();
            assert_eq!(output, "123abcd456");
            assert_eq!(rest, "");
        }
    }
}
