use crate::utils::{count_eq, order_by_len_asc};
use crate::{DistanceMetric, Jaro};

use std::cmp;

#[derive(Debug, Clone)]
pub struct WinklerConfig {
    /// Scaling factor. Default to 0.1
    scaling: f64,
    /// Boost threshold. Default to 0.7
    threshold: f64,
    /// max length of common prefix. Default to 4
    max_length: usize,
}

impl WinklerConfig {
    /// # Panics
    ///
    /// Panics if the scaling factor times maxlength of common prefix is higher
    /// than one.
    pub fn new(scaling: f64, threshold: f64, max_length: usize) -> Self {
        assert!(scaling * max_length as f64 <= 1.);
        Self {
            scaling,
            threshold,
            max_length,
        }
    }
}

impl Default for WinklerConfig {
    fn default() -> Self {
        Self {
            scaling: 0.1,
            threshold: 0.7,
            max_length: 4,
        }
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
    /// The base distance to modify.
    inner: D,
    /// Coefficients for winkler modification.
    config: WinklerConfig,
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
        let a = a.into_iter();
        let b = b.into_iter();

        let mut score = self.inner.distance(a.clone(), b.clone()).into();

        if score <= 1. - self.config.threshold {
            let eq_prefix = count_eq(a, b);
            score -=
                cmp::min(eq_prefix, self.config.max_length) as f64 * self.config.scaling * score;
        }

        score
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

impl Default for Winkler<Jaro> {
    fn default() -> Self {
        Self {
            inner: Jaro,
            config: Default::default(),
        }
    }
}
