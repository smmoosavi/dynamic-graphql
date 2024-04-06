use crate::content::peek::Peek;
use crate::content::Content;

pub trait Delete {
    fn delete(&self, content: &mut Content) -> Result<(), String>;
}

impl<T> Delete for T
where
    T: Peek,
{
    fn delete(&self, content: &mut Content) -> Result<(), String> {
        let (_found, rest) = self.peek(&content.rest)?;
        content.rest = rest.to_string();
        Ok(())
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
            content.delete(&len(3)).unwrap();
            assert_eq!(content.output, "");
            assert_eq!(content.rest, "abcd456");
        }
        #[test]
        fn multiple() {
            let mut content = Content::new("123abcd456".to_string());
            content.delete(&len(3)).unwrap();
            content.delete(&len(4)).unwrap();
            assert_eq!(content.output, "");
            assert_eq!(content.rest, "456");
        }

        #[test]
        fn mixed() {
            let mut content = Content::new("123abcd456".to_string());
            content.seek(&len(3)).unwrap();
            content.delete(&len(4)).unwrap();
            assert_eq!(content.output, "123");
            assert_eq!(content.rest, "456");
        }
    }
}
