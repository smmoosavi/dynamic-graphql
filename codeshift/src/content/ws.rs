use super::Peek;

/// Whitespaces
pub struct Ws;

impl Peek for Ws {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        let len = content.chars().take_while(|c| c.is_whitespace()).count();
        Ok((&content[..len], &content[len..]))
    }
}

pub fn ws() -> Ws {
    Ws
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;
        #[test]
        fn not_at_start() {
            let input = "123    abcd456";
            let (output, rest) = ws().peek(input).unwrap();
            assert_eq!(output, "");
            assert_eq!(rest, "123    abcd456");
        }
        #[test]
        fn at_start() {
            let input = "    abcd456";
            let (output, rest) = ws().peek(input).unwrap();
            assert_eq!(output, "    ");
            assert_eq!(rest, "abcd456");
        }

        #[test]
        fn tab_and_newline() {
            let input = "\t\n abcd456";
            let (output, rest) = ws().peek(input).unwrap();
            assert_eq!(output, "\t\n ");
            assert_eq!(rest, "abcd456");
        }
    }
}
