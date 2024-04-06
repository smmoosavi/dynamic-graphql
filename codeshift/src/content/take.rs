use crate::content::peek::Peek;
use crate::content::Content;

pub trait Take {
    fn take(&self, content: &mut Content) -> Result<String, String>;
}

impl<T> Take for T
where
    T: Peek,
{
    fn take(&self, content: &mut Content) -> Result<String, String> {
        let (found, rest) = self.peek(&content.rest)?;
        let found = found.to_string();
        content.rest = rest.to_string();
        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod delete {
        use super::*;
        use crate::content::len;

        #[test]
        fn start() {
            let mut content = Content::new("123abcd456".to_string());
            let taken = content.take(&len(3)).unwrap();
            assert_eq!(taken, "123");
            assert_eq!(content.output, "");
            assert_eq!(content.rest, "abcd456");
        }
        #[test]
        fn mixed() {
            let mut content = Content::new("123abcd456".to_string());
            content.seek(&len(3)).unwrap();
            let taken = content.take(&len(4)).unwrap();
            assert_eq!(taken, "abcd");
            assert_eq!(content.output, "123");
            assert_eq!(content.rest, "456");
        }
    }
}
