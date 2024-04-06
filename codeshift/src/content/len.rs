use super::Peek;

pub struct Len(pub usize);

impl Peek for Len {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        let (output, rest) = content.split_at(self.0.min(content.len()));
        Ok((output, rest))
    }
}

pub fn len(l: usize) -> Len {
    Len(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;
        #[test]
        fn short() {
            let input = "123abcd456";
            let (output, rest) = len(3).peek(input).unwrap();
            assert_eq!(output, "123");
            assert_eq!(rest, "abcd456");
        }

        #[test]
        fn long() {
            let input = "123abcd456";
            let (output, rest) = len(20).peek(input).unwrap();
            assert_eq!(output, "123abcd456");
            assert_eq!(rest, "");
        }
    }
}
