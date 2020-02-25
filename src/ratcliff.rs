use crate::DistanceMetric;

/// The distance between two strings is defined as one minus  the number of
/// matching characters divided by the total number of characters in the two
/// strings. Matching characters are those in the longest common subsequence
/// plus, recursively, matching characters in the unmatched region on either
/// side of the longest common subsequence.
pub struct RatcliffObershelp;

impl DistanceMetric for RatcliffObershelp {
    type Dist = f64;

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        let a = a.into_iter();
        let b = b.into_iter();
        let len_a = a.clone().count();
        let len_b = b.clone().count();

        let matched = SequenceMatcher::new(a, b, len_a, len_b).match_sequences();

        if len_a + len_b == 0 {
            0.
        } else {
            1.0 - 2. * matched as f64 / (len_a + len_b) as f64
        }
    }
    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        self.distance(a, b)
    }
}

struct SequenceMatcher<S, T>
where
    S: Iterator + Clone,
    T: Iterator + Clone,
    <S as Iterator>::Item: PartialEq<<T as Iterator>::Item>,
{
    s1: S,
    s2: T,
    /// The length of iterator s1
    len1: usize,
    /// The length of iterator s2
    len2: usize,
    /// Index where the to start matching on s1
    start1: usize,
    /// Index where the to start matching on s2
    start2: usize,
}

impl<S, T> SequenceMatcher<S, T>
where
    S: Iterator + Clone,
    T: Iterator + Clone,
    <S as Iterator>::Item: PartialEq<<T as Iterator>::Item>,
{
    #[inline]
    fn new(s1: S, s2: T, len1: usize, len2: usize) -> Self {
        Self {
            len1,
            len2,
            s1,
            s2,
            start1: 0,
            start2: 0,
        }
    }

    /// Finds the longest substr of both iters the recursively find the
    /// longest substr of both tails.
    fn match_sequences(self) -> usize {
        let subseq = longest_common_subsequence(
            self.s1.clone().skip(self.start1).take(self.len1),
            self.s2.clone().skip(self.start2).take(self.len2),
            self.len1,
            self.len2,
        );

        if subseq.is_empty() {
            // stop if there is no common substring
            return 0;
        }

        let mut ctn = subseq.len;

        // add the longest common substring that happens before
        let before = SequenceMatcher {
            s1: self.s1.clone(),
            s2: self.s2.clone(),
            len1: subseq.s1_idx,
            len2: subseq.s2_idx,
            start1: self.start1,
            start2: self.start2,
        };
        ctn += before.match_sequences();

        // add the longest common substring that happens after
        let after = SequenceMatcher {
            s1: self.s1,
            s2: self.s2,
            len1: self.len1 - (subseq.s1_idx + subseq.len),
            len2: self.len2 - (subseq.s2_idx + subseq.len),
            start1: self.start1 + subseq.s1_idx + subseq.len,
            start2: self.start2 + subseq.s2_idx + subseq.len,
        };
        ctn + after.match_sequences()
    }
}

struct CommonSubseq {
    /// Start index of the pattern in iter 1
    s1_idx: usize,
    /// Start index of the pattern in iter 2
    s2_idx: usize,
    /// Length of the common sequence
    len: usize,
}

impl CommonSubseq {
    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Find the longest common substr of both iterators.
fn longest_common_subsequence<S, T>(s1: S, s2: T, _s1_len: usize, s2_len: usize) -> CommonSubseq
where
    S: Iterator + Clone,
    T: Iterator + Clone,
    <S as Iterator>::Item: PartialEq<<T as Iterator>::Item>,
{
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
    CommonSubseq {
        s1_idx: start1,
        s2_idx: start2,
        len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_subseq() {
        let s1 = "aaabc";
        let s2 = "axaxbcaaa";
        let subs = longest_common_subsequence(
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
    fn empty_common_subseq() {
        let subs = longest_common_subsequence("".chars(), "kitten".chars(), 0, 6);
        assert!(subs.is_empty());
    }

    #[test]
    fn ratcliff_obershelp() {
        assert_eq!(RatcliffObershelp.str_distance("", "kitten"), 1.0);
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.str_distance("alexandre", "aleksander")
            ),
            "0.263158"
        );
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.str_distance("pennsylvania", "pencilvaneya")
            ),
            "0.333333"
        );
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.str_distance("abandonned", "abandoned")
            ),
            "0.052632"
        );
        assert_eq!(
            format!(
                "{:.6}",
                RatcliffObershelp.str_distance("Result.unwrap()", "Result.unwarp()")
            ),
            "0.066667"
        );
        assert_eq!(
            format!("{:.6}", RatcliffObershelp.str_distance("ahppen", "happen")),
            "0.166667"
        );
    }
}
