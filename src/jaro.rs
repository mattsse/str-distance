use std::cmp;

use crate::modifiers::Winkler;
use crate::utils::order_by_len_asc;
use crate::DistanceMetric;

pub struct Jaro;

impl DistanceMetric for Jaro {
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
        let s1: Vec<_> = a.into_iter().collect();
        let s2: Vec<_> = b.into_iter().collect();

        let s1_len = s1.len();
        let s2_len = s2.len();

        // edge cases
        if s1_len + s2_len == 0 {
            return 0.0;
        } else if cmp::min(s1_len, s2_len) == 0 {
            return 1.0;
        } else if s1_len + s2_len == 2 {
            return if s1[0] == s2[0] { 0. } else { 1. };
        }

        let max_dist = cmp::max(s1_len, s2_len) / 2 - 1;
        let mut s1_matches = vec![false; s1_len];
        let mut s2_matches = vec![false; s2_len];
        let mut matches = 0usize;

        for i in 0..s1_len {
            let start = cmp::max(0, i as isize - max_dist as isize) as usize;
            let end = cmp::min(i + max_dist + 1, s2_len);
            for j in start..end {
                if !s2_matches[j] && s1[i] == s2[j] {
                    s1_matches[i] = true;
                    s2_matches[j] = true;
                    matches += 1;
                    break;
                }
            }
        }
        if matches == 0 {
            return 1.;
        }
        let mut transpositions = 0.0;
        let mut k = 0;
        for i in 0..s1_len {
            if s1_matches[i] {
                while !s2_matches[k] {
                    k += 1;
                }
                if s1[i] != s2[k] {
                    transpositions += 0.5;
                }
                k += 1;
            }
        }
        let m = matches as f64;
        1. - (m / s1_len as f64 + m / s2_len as f64 + (m - transpositions) / m) / 3.0
    }

    fn str_distance<S, T>(&self, s1: S, s2: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let (s1, s2) = order_by_len_asc(s1.as_ref(), s2.as_ref());
        self.distance(s1.chars(), s2.chars())
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

/// Jaro Distance with winkler modification.
pub type JaroWinkler = Winkler<Jaro>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jaro() {
        assert_eq!(Jaro.str_distance("", ""), 0.0);
        assert_eq!(Jaro.str_distance("foo", "foo"), 0.0);
        assert_eq!(
            format!("{:.6}", Jaro.str_distance("foo", "foo ")),
            "0.083333"
        );
        assert_eq!(
            format!(
                "{:.6}",
                Jaro.str_distance("D N H Enterprises Inc", "D &amp; H Enterprises, Inc.")
            ),
            "0.177293"
        );
        assert_eq!(
            format!("{:.6}", Jaro.str_distance("elephant", "hippo")),
            "0.558333"
        );
    }

    #[test]
    fn winkler() {
        assert_eq!(
            format!(
                "{:.6}",
                JaroWinkler::default().str_distance("martha", "marhta")
            ),
            "0.038889"
        );
    }
}
