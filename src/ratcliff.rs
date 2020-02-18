use crate::Distance;
use std::collections::HashSet;
use std::iter::{Skip, Take};
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

        let matched = SequenceMatcher::new(s1.chars(), s2.chars(), len1, len2).match_sequences();

        if len1 + len2 == 0 {
            0.
        } else {
            1.0 - 2. * matched as f64 / (len1 + len2) as f64
        }
    }
}

struct SequenceMatcher<'a> {
    s1: Chars<'a>,
    s2: Chars<'a>,
    /// The length of iterator s1
    len1: usize,
    /// The length of iterator s2
    len2: usize,
    /// Index where the to start matching on s1
    start1: usize,
    /// Index where the to start matching on s2
    start2: usize,
}

impl<'a> SequenceMatcher<'a> {
    #[inline]
    fn new(s1: Chars<'a>, s2: Chars<'a>, len1: usize, len2: usize) -> Self {
        Self {
            len1,
            len2,
            s1,
            s2,
            start1: 0,
            start2: 0,
        }
    }

    /// Finds the longest substr of both `Chars` the recursively find the
    /// longest substr of both tails.
    fn match_sequences(self) -> usize {
        let substr = longest_common_substring(
            self.s1.clone().skip(self.start1).take(self.len1),
            self.s2.clone().skip(self.start2).take(self.len2),
            self.len1,
            self.len2,
        );

        if substr.is_empty() {
            // stop if there is no common substring
            return 0;
        }

        let mut ctn = substr.len;

        // add the longest common substring that happens before
        let before = SequenceMatcher {
            s1: self.s1.clone(),
            s2: self.s2.clone(),
            len1: substr.s1_idx,
            len2: substr.s2_idx,
            start1: self.start1,
            start2: self.start2,
        };
        ctn += before.match_sequences();

        // add the longest common substring that happens after
        let after = SequenceMatcher {
            s1: self.s1,
            s2: self.s2,
            len1: self.len1 - (substr.s1_idx + substr.len),
            len2: self.len2 - (substr.s2_idx + substr.len),
            start1: self.start1 + substr.s1_idx + substr.len,
            start2: self.start2 + substr.s2_idx + substr.len,
        };
        ctn + after.match_sequences()
    }
}

struct CommonSubstr {
    /// Start index of the pattern in str 1
    s1_idx: usize,
    /// Start index of the pattern in str 2
    s2_idx: usize,
    /// Length of the common pattern
    len: usize,
}

impl CommonSubstr {
    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Find the longest common substr of both iterators.
fn longest_common_substring<Iter: Iterator<Item = char> + Clone>(
    s1: Iter,
    s2: Iter,
    s1_len: usize,
    s2_len: usize,
) -> CommonSubstr {
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
        CommonSubstr {
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
        let subs = longest_common_substring(
            s1.chars(),
            s2.chars(),
            s1.chars().count(),
            s2.chars().count(),
        );
        assert_eq!(subs.s1_idx, 0);
        assert_eq!(subs.s2_idx, 6);
        assert_eq!(subs.len, 3);
    }

    #[test]
    fn empty_common_substr() {
        let subs = longest_common_substring("".chars(), "kitten".chars(), 0, 6);
        assert!(subs.is_empty());
    }

    #[test]
    fn ratcliff_obershelp() {
        assert_eq!(RatcliffObershelp.distance("", "kitten"), 1.0);
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.distance("alexandre", "aleksander")
            ),
            "0.263158"
        );
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.distance("pennsylvania", "pencilvaneya")
            ),
            "0.333333"
        );
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.distance("abandonned", "abandoned")
            ),
            "0.052632"
        );
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.distance("Result.unwrap()", "Result.unwarp()")
            ),
            "0.066667"
        );
        assert_eq!(
            format!("{:.6}", RatcliffObershelp.distance("ahppen", "happen")),
            "0.166667"
        );
    }
}
