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
    fn split_at_nth<'a>(&self, pat: char, count: usize) -> Option<(&'a str, &'a str)>
    where
        Self: 'a;
}

impl SplitExt for &str {
    fn split_at_nth<'a>(&self, pat: char, count: usize) -> Option<(&'a str, &'a str)>
    where
        Self: 'a,
    {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_nth_occurrence() {
        let input = "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)(-0.000016 -0.000017 19.995882)";

        let (before, after) = input.split_at_nth(')', 3).unwrap();
        assert_eq!(before, "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)");
        assert_eq!(after, "(-0.000016 -0.000017 19.995882)");
    }

    #[test]
    fn test_split_at_nth_occurrence_not_found() {
        let input = "(0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)(-0.000016 -0.000017 19.995882)";

        let result = input.split_at_nth(')', 5);
        assert_eq!(result, None);
    }
}
