use crate::content::Peek;

pub struct Lookaheads<'s> {
    rest: &'s str,
}

impl Lookaheads<'_> {
    pub fn new(s: &str) -> Lookaheads {
        Lookaheads { rest: s }
    }

    pub fn seek(&mut self, seeker: &dyn Peek) -> Result<&mut Self, String> {
        let (_found, rest) = seeker.peek(self.rest)?;
        self.rest = rest;
        Ok(self)
    }
}
