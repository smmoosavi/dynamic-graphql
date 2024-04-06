pub trait Peek {
    /// returns peeked and rest
    /// it should be used only in forward direction
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String>;
}
