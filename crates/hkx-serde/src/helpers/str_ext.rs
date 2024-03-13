/// A trait for extending string functionality related to parsing Havok Engine XML.
pub trait SplitExt {
    /// Split a string at the nth occurrence of the specified character.
    ///
    /// ```
    /// let input = "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)(-0.000016 -0.000017 19.995882)";
    /// let (before, after) = input.split_at_nth(')', 3).unwrap();
    /// assert_eq!(before, "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)");
    /// assert_eq!(after, "(-0.000016 -0.000017 19.995882)");
    /// ```
    fn split_at_nth(&self, pat: char, count: usize) -> Option<(&str, &str)>;

    /// Split a string at the nth occurrence of the specified character sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// let input = "a b c d e f";
    /// let mut parts = input.chunk(' ', 2);
    /// assert_eq!(parts.next(), Some("a b"));
    /// assert_eq!(parts.next(), Some("c d"));
    /// assert_eq!(parts.next(), Some("e f"));
    /// assert_eq!(parts.next(), None);
    /// ```
    fn chunk(&self, separator: char, chunk_size: usize) -> Vec<&str>;
}

impl SplitExt for &str {
    fn split_at_nth(&self, pat: char, count: usize) -> Option<(&str, &str)> {
        let mut occurrence_count = 0;
        for (i, c) in self.char_indices() {
            if c == pat {
                occurrence_count += 1;
                if occurrence_count == count {
                    return Some((&self[..=i], &self[i..]));
                }
            }
        }
        None
    }

    fn chunk(&self, separator: char, chunk_size: usize) -> Vec<&str> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut count = 0;

        for (idx, ch) in self.char_indices() {
            if ch == separator {
                count += 1;
                if count == chunk_size {
                    chunks.push(&self[start..idx]);
                    start = idx + ch.len_utf8();
                    count = 0;
                }
            }
        }

        if start < self.len() {
            chunks.push(&self[start..]);
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_split_at_nth() {
        let input = "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)(-0.000016 -0.000017 19.995882)";

        let (before, after) = input.split_at_nth(')', 3).unwrap();
        assert_eq!(before, "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)");
        assert_eq!(after, "(-0.000016 -0.000017 19.995882)");
    }

    #[test]
    fn test_split_at_nth_not_found() {
        let input = "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)(-0.000016 -0.000017 19.995882)";

        let result = input.split_at_nth(')', 5);
        assert_eq!(result, None);
    }

    #[test]
    fn test_split_chunk() {
        let input = "a b c d e f";
        let mut parts = input.chunk(' ', 2).into_iter();
        assert_eq!(parts.next(), Some("a b"));
        assert_eq!(parts.next(), Some("c d"));
        assert_eq!(parts.next(), Some("e f"));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn test_split_chunk_not_found() {
        let input = "a b c d e f";
        let mut parts = input.chunk(' ', 10).into_iter();
        assert_eq!(parts.next(), Some("a b c d e f"));
        assert_eq!(parts.next(), None);
    }
}
