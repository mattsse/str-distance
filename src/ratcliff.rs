use crate::Distance;

/// The distance between two strings is defined as one minus  the number of
/// matching characters divided by the total number of characters in the two
/// strings. Matching characters are those in the longest common subsequence
/// plus, recursively, matching characters in the unmatched region on either
/// side of the longest common subsequence.
pub struct RatcliffObershelp;

impl Distance for RatcliffObershelp {
    #[inline]
    fn distance<S, T>(&self, s1: S, s2: T) -> usize
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        unimplemented!()
    }
}

struct CommonPattern {
    /// Start index of the pattern in str 1
    s1_idx: usize,
    /// Start index of the pattern in str 2
    s2_idx: usize,
    /// Length of the common pattern
    len: usize,
}

impl CommonPattern {
    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn longest_common_pattern<Iter: Iterator<Item = char> + Clone>(
    s1: Iter,
    s2: Iter,
    s1_len: usize,
    s2_len: usize,
) -> CommonPattern {
    if s1_len > s2_len {
        longest_common_pattern(s2, s1, s2_len, s1_len)
    } else {
        let mut p = vec![0usize; s2_len];
        let (mut start1, mut start2, mut len) = (0, 0, 0);
        for (s1_idx, c1) in s1.enumerate() {
            let mut oldp = 0;
            for (s2_idx, c2) in s2.clone().enumerate() {
                let mut newp = 0;
                if c1 == c2 {
                    newp = if oldp > 0 { oldp } else { s2_idx };
                    let current_len = s2_idx + 1 - newp;
                    if current_len > len {
                        start1 = s1_idx + 1 - current_len;
                        start2 = newp;
                        len = current_len;
                    }
                }
                oldp = p[s2_idx];
                p[s2_idx] = newp;
            }
        }
        CommonPattern {
            s1_idx: start1,
            s2_idx: start2,
            len,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_substr() {
        let s1 = "aaabc";
        let s2 = "axaxbcaaa";
        let pat = longest_common_pattern(s1.chars(), s2.chars(), s1.chars().count(), s2.chars().count());
        assert_eq!(pat.s1_idx, 0);
        assert_eq!(pat.s2_idx, 6);
        assert_eq!(pat.len, 3);
    }
}
