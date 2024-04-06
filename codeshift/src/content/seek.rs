use crate::content::peek::Peek;
use crate::content::Content;

pub trait Seek {
    fn seek(&self, content: &mut Content) -> Result<(), String>;
}

impl<T> Seek for T
where
    T: Peek,
{
    fn seek(&self, content: &mut Content) -> Result<(), String> {
        let (found, rest) = self.peek(&content.rest)?;
        content.output.push_str(found);
        content.rest = rest.to_string();
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    mod seek {
        use super::*;
        use crate::content::len;

        #[test]
        fn start() {
            let mut content = Content::new("123abcd456".to_string());
            content.seek(&len(3)).unwrap();
            assert_eq!(content.output, "123");
            assert_eq!(content.rest, "abcd456");
        }
        #[test]
        fn multiple() {
            let mut content = Content::new("123abcd456".to_string());
            content.seek(&len(3)).unwrap();
            content.seek(&len(4)).unwrap();
            assert_eq!(content.output, "123abcd");
            assert_eq!(content.rest, "456");
        }
    }
}
