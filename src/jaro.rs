use crate::utils::{count_eq, order_by_len_asc};
use crate::DistanceMetric;
use std::cmp;
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
        let mut m = 0usize;

        for i in 0..s1_len {
            let start = cmp::max(0, i as isize - max_dist as isize) as usize;
            let end = cmp::min(i + max_dist + 1, s2_len);
            for j in start..end {
                if !s2_matches[j] && s1[i] == s2[j] {
                    s1_matches[i] = true;
                    s2_matches[j] = true;
                    m += 1;
                    break;
                }
            }
        }
        if m == 0 {
            return 1.;
        }
        let mut t = 0.0;
        let mut k = 0;
        for i in 0..s1_len {
            if s1_matches[i] {
                while !s2_matches[k] {
                    k += 1;
                }
                if s1[i] != s2[k] {
                    t += 0.5;
                }
                k += 1;
            }
        }
        let m = m as f64;
        1. - (m / s1_len as f64 + m / s2_len as f64 + (m - t) / m) / 3.0
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

        for (s1_idx, c1) in s1.chars().enumerate() {
            let min_bound =
                // prevent integer wrapping
                if s1_idx > max_dist {
                    0.max( s1_idx - max_dist)
                } else {
                    0
                };

            let max_bound = (s2_len - 1).min(s1_idx + max_dist);

            if min_bound > max_bound {
                continue;
            }

            for (s2_idx, c2) in s2.chars().enumerate() {
                if min_bound <= s2_idx && s2_idx <= max_bound && c1 == c2 && !flags[s2_idx] {
                    flags[s2_idx] = true;
                    matches += 1.0;

                    if s2_idx < c2_match_idx {
                        transpositions += 1.0;
                    }
                    c2_match_idx = s2_idx;
                    break;
                }
            }
        }

        if matches == 0.0 {
            0.0
        } else {
            (1.0 / 3.0)
                * ((matches / s1_len as f64)
                    + (matches / s2_len as f64)
                    + ((matches - transpositions) / matches))
        }
    }
}

pub struct JaroWinkler;

impl DistanceMetric for JaroWinkler {
    type Dist = f64;
    #[inline]
    fn str_distance<S, T>(&self, s1: S, s2: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let s1 = s1.as_ref();
        let s2 = s2.as_ref();
        let mut dist = Jaro.str_distance(s1, s2);

        let prefix_ctn = count_eq(s1.chars(), s2.chars());

        dist += 0.1 * prefix_ctn as f64 * (1.0 - dist);

        if dist <= 1.0 {
            0.0
        } else {
            1.0
        }
    }
}

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
}
