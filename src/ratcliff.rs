use crate::Distance;
use std::collections::HashSet;
use std::str::Chars;

/// The distance between two strings is defined as one minus  the number of
/// matching characters divided by the total number of characters in the two
/// strings. Matching characters are those in the longest common subsequence
/// plus, recursively, matching characters in the unmatched region on either
/// side of the longest common subsequence.
pub struct RatcliffObershelp;

impl RatcliffObershelp {
    #[inline]
    fn distance<S, T>(&self, s1: S, s2: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let s1 = s1.as_ref();
        let s2 = s2.as_ref();
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();

        let m = SequenceMatcher::new(s1.chars(), s2.chars(), len1, len2);

        let mut indices = HashSet::default();
        m.match_sequences(&mut indices);

        let matched: usize = indices.into_iter().map(|(_, _, i)| i).sum();

        if len1 + len2 == 0 {
            0.
        } else {
            1.0 - 2. * matched as f64 / (len1 + len2) as f64
        }
    }
}

struct SequenceMatcher<Iter: Iterator<Item = char> + Clone> {
    s1: Iter,
    s2: Iter,
    len1: usize,
    len2: usize,
    start1: usize,
    start2: usize,
}

impl<Iter> SequenceMatcher<Iter>
where
    Iter: Iterator<Item = char> + Clone,
{
    #[inline]
    fn new(s1: Iter, s2: Iter, len1: usize, len2: usize) -> Self {
        Self {
            len1,
            len2,
            s1,
            s2,
            start1: 0,
            start2: 0,
        }
    }

    fn match_sequences(self, indices: &mut HashSet<(usize, usize, usize)>) {
        let a = longest_common_substring(self.s1.clone(), self.s2.clone(), self.len1, self.len2);
        if a.is_empty() {
            // stop if there is no common substring
            return;
        }
        // cache the common substring
        indices.insert((
            a.s1_idx + self.start1 - 1,
            a.s2_idx + self.start2 - 1,
            a.len,
        ));

        // add the longest common substring that happens before
        let before = SequenceMatcher {
            s1: self.s1.clone().take(a.s1_idx),
            s2: self.s2.clone().take(a.s2_idx),
            len1: a.s1_idx,
            len2: a.s2_idx,
            start1: self.start1,
            start2: self.start2,
        };
        before.match_sequences(indices);

        // add the longest common substring that happens after
        let after = SequenceMatcher {
            s1: self.s1.skip(a.s1_idx + a.len),
            s2: self.s2.skip(a.s2_idx + a.len),
            len1: self.len1 - (a.s1_idx + a.len),
            len2: self.len2 - (a.s2_idx + a.len),
            start1: self.start1 + a.s1_idx + a.len,
            start2: self.start2 + a.s2_idx + a.len,
        };
        after.match_sequences(indices);
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

/// Find the longest common substring of both iterators.
fn longest_common_substring<Iter: Iterator<Item = char> + Clone>(
    s1: Iter,
    s2: Iter,
    s1_len: usize,
    s2_len: usize,
) -> CommonPattern {
    if s1_len > s2_len {
        let mut seq = longest_common_substring(s2, s1, s2_len, s1_len);
        std::mem::swap(&mut seq.s1_idx, &mut seq.s2_idx);
        seq
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
        let pat = longest_common_substring(
            s1.chars(),
            s2.chars(),
            s1.chars().count(),
            s2.chars().count(),
        );
        assert_eq!(pat.s1_idx, 0);
        assert_eq!(pat.s2_idx, 6);
        assert_eq!(pat.len, 3);
    }
}
