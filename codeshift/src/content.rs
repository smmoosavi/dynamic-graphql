mod after;
mod delete;
mod end;
mod len;
mod lookaheads;
mod peek;
mod seek;
mod str;
mod take;
mod until;
mod ws;

pub use after::after;
pub use len::len;
pub use lookaheads::Lookaheads;
pub use peek::Peek;
pub use seek::Seek;
pub use str::str;
pub use until::until;
pub use ws::ws;
pub struct Content {
    output: String,
    rest: String,
}

fn found(rest: &str, pattern: &str) -> String {
    let len = pattern.len() + 10;
    if rest.len() < len {
        rest.to_string()
    } else {
        rest.chars().take(len).collect::<String>() + "..."
    }
}

impl Content {
    pub fn new(s: String) -> Self {
        Content {
            output: String::new(),
            rest: s,
        }
    }

    pub fn seek(&mut self, seeker: &dyn Seek) -> Result<(), String> {
        seeker.seek(self)
    }

    pub fn delete(&mut self, deleter: &dyn delete::Delete) -> Result<(), String> {
        deleter.delete(self)
    }

    pub fn take(&mut self, taker: &dyn take::Take) -> Result<String, String> {
        taker.take(self)
    }
    pub fn insert(&mut self, s: &str) {
        self.output.push_str(s);
    }

    pub fn lookaheads(&self) -> lookaheads::Lookaheads {
        lookaheads::Lookaheads::new(&self.rest)
    }
    pub fn is_done(&self) -> bool {
        self.rest.is_empty()
    }

    pub fn finish(&self) -> Result<String, String> {
        if self.rest.is_empty() {
            Ok(self.output.clone())
        } else {
            Err(format!("Unprocessed content: {}", found(&self.rest, ""),))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod insert {
        use super::*;
        use crate::content::len;

        #[test]
        fn start() {
            let mut content = Content::new("123abcd456".to_string());
            content.insert("abc");
            content.seek(&len(3)).unwrap();
            assert_eq!(content.output, "abc123");
            assert_eq!(content.rest, "abcd456");
        }
        #[test]
        fn middle() {
            let mut content = Content::new("123abcd456".to_string());
            content.seek(&len(3)).unwrap();
            content.insert("abc");
            assert_eq!(content.output, "123abc");
            assert_eq!(content.rest, "abcd456");
        }
    }
}
