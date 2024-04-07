use super::Peek;

/// Whitespaces
pub struct Paren;

impl Peek for Paren {
    fn peek<'c>(&self, content: &'c str) -> Result<(&'c str, &'c str), String> {
        if !content.starts_with('(') {
            return Ok(("", content));
        }
        let mut depth = 0;
        let mut part1_end = 0;

        for (i, c) in content.char_indices() {
            match c {
                '(' => {
                    depth += 1;
                }
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        part1_end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }

        if depth != 0 {
            return Err("unmatched parentheses".to_string());
        }

        let part2_start = part1_end;

        let part1 = &content[..part1_end];
        let part2 = &content[part2_start..];

        Ok((part1, part2))
    }
}

pub fn paren() -> Paren {
    Paren
}

#[cfg(test)]
mod tests {
    use super::*;

    mod peek {
        use super::*;
        #[test]
        fn not_at_start() {
            let input = "aa (bb) cc";
            let (output, rest) = paren().peek(input).unwrap();
            assert_eq!(output, "");
            assert_eq!(rest, "aa (bb) cc");
        }
        #[test]
        fn at_start() {
            let input = "(bb) cc";
            let (output, rest) = paren().peek(input).unwrap();
            assert_eq!(output, "(bb)");
            assert_eq!(rest, " cc");
        }

        #[test]
        fn nested() {
            let input = "(bb(x)) cc";
            let (output, rest) = paren().peek(input).unwrap();
            assert_eq!(output, "(bb(x))");
            assert_eq!(rest, " cc");
        }

        #[test]
        fn nested_multiple() {
            let input = "(bb(x)(y)) cc";
            let (output, rest) = paren().peek(input).unwrap();
            assert_eq!(output, "(bb(x)(y))");
            assert_eq!(rest, " cc");
        }

        #[test]
        fn multiple() {
            let input = "(bb(x)) (yy) cc";
            let (output, rest) = paren().peek(input).unwrap();
            assert_eq!(output, "(bb(x))");
            assert_eq!(rest, " (yy) cc");
        }

        #[test]
        fn not_closed() {
            let input = "(bb cc";
            let err = paren().peek(input).unwrap_err();
            assert_eq!(err, "unmatched parentheses");
        }
    }
}
