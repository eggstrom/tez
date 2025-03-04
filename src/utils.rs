use std::ops::Deref;

pub trait StrExt {
    /// Finds the last occurrence of a char within the first group, where a
    /// group is multiple of the same char, optionally separated by whitespace.
    fn find_last_adjacent(&self, ch: char) -> Option<usize>;
}

impl<T> StrExt for T
where
    T: Deref<Target = str>,
{
    fn find_last_adjacent(&self, ch: char) -> Option<usize> {
        let mut last_ch = None;
        for (i, c) in self.char_indices() {
            match c {
                _ if c == ch => last_ch = Some(i),
                _ if !c.is_whitespace() && last_ch.is_some() => break,
                _ => (),
            }
        }
        last_ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_last_adjacent() {
        assert_eq!("aa".find_last_adjacent('a'), Some(1));
        assert_eq!("aabaa".find_last_adjacent('a'), Some(1));
        assert_eq!("baa".find_last_adjacent('a'), Some(2));
        assert_eq!("aa aa".find_last_adjacent('a'), Some(4));
    }
}
