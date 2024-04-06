use super::Peek;

pub struct End;

impl Peek for End {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        Ok((content, ""))
    }
}

pub fn end() -> End {
    End
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;
        #[test]
        fn short() {
            let input = "123abcd456";
            let (output, rest) = end().peek(input).unwrap();
            assert_eq!(output, "123abcd456");
            assert_eq!(rest, "");
        }
    }
}
