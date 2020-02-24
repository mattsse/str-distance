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

/// `Winkler` modifies a [`DistanceMetric`]'s distance to decrease the distance
/// between  two strings, when their original distance is below some
/// `threshold`. The boost is equal to `min(l,  maxlength) * p * dist` where `l`
/// denotes the length of their common prefix and `dist` denotes the original
/// distance. The Winkler adjustment was originally defined for the [`Jaro`]
/// similarity score but is here defined it for any distance.
#[derive(Debug, Clone)]
pub struct Winkler<D: DistanceMetric> {
    inner: D,
    config: WinklerConfig,
    // TODO add max distance
}

#[derive(Debug, Clone)]
pub struct WinklerConfig {
    /// scaling factor. Default to 0.1
    pub p: f64,
    /// boost threshold. Default to 0.7
    pub threshold: f64,
    /// max length of common prefix. Default to 4
    pub max_length: usize,
}

impl<D: DistanceMetric> Winkler<D> {
    pub fn new(inner: D) -> Self {
        Self {
            inner,
            config: Default::default(),
        }
    }

    pub fn with_config(inner: D, config: WinklerConfig) -> Self {
        Self { inner, config }
    }
}

impl Default for WinklerConfig {
    fn default() -> Self {
        Self {
            p: 0.1,
            threshold: 0.7,
            max_length: 4,
        }
    }
}

impl Default for Winkler<Jaro> {
    fn default() -> Self {
        Self {
            inner: Jaro,
            config: Default::default(),
        }
    }
}

impl<D> DistanceMetric for Winkler<D>
where
    D: DistanceMetric,
    <D as DistanceMetric>::Dist: Into<f64>,
{
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
        // scaling factor times maxlength of common prefix must be lower than one
        assert!(self.config.p * self.config.max_length as f64 <= 1.);

        // let a = a.into_iter();
        // let b = b.into_iter();
        // let mut dist = Jaro.distance(a.clone(),b.clone());
        // let prefix_ctn = count_eq(a, b);
        //
        // dist += 0.1 * prefix_ctn as f64 * (1.0 - dist);
        //
        // if dist <= 1.0 {
        //     0.0
        // } else {
        //     1.0
        // }
        unimplemented!()
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
