//! Compute distances between strings

// TODO add some remarks about coefficient vs distance (= 1 - coefficient):
// in contrast to the coefficient/index of a metric, distance is a measure of dissimilarity
// see for example: https://en.wikipedia.org/wiki/Jaccard_index

#![forbid(unsafe_code)]
// #![allow(unused)]

use std::ops::Deref;

pub use jaro::{Jaro, JaroWinkler};
pub use levenshtein::{DamerauLevenshtein, Levenshtein};
pub use qgram::{Cosine, Jaccard, Overlap, QGram, SorensenDice};
pub use ratcliff::RatcliffObershelp;
pub use token::TokenSet;

pub mod jaro;
pub mod levenshtein;
pub mod qgram;
pub mod ratcliff;
pub mod token;
mod utils;

/// Evaluates the distance between two strings based on the provided
/// [`crate::DistanceMetric`].
///
/// # Examples
///
/// ```
/// # use str_distance::{Levenshtein, str_distance, SorensenDice, TokenSet, RatcliffObershelp, DistanceValue};
/// assert_eq!(str_distance("kitten", "sitting", Levenshtein::default()), DistanceValue::Exact(3));
/// assert_eq!(str_distance("kitten", "sitting", Levenshtein::with_max_distance(1)), DistanceValue::Exceeded(1));
/// assert_eq!(str_distance("nacht", "night", SorensenDice::default()), 0.75);
/// assert_eq!(str_distance("Real Madrid vs FC Barcelona", "Barcelona vs Real Madrid",
/// TokenSet::new(RatcliffObershelp)), 0.0);
/// ```
pub fn str_distance<S, T, D>(a: S, b: T, dist: D) -> <D as DistanceMetric>::Dist
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: DistanceMetric,
{
    dist.str_distance(a, b)
}

/// Evaluates the normalized distance between two strings based on the provided
/// [`crate::DistanceMetric`], so that it returns always a f64 between 0 and 1.
/// A value of '0.0' corresponds to the "zero distance", both strings are
/// considered equal by means of the metric, whereas a value of '1.0'
/// corresponds to the maximum distance that can exist between the strings.
///
/// # Remark
///
/// The distance between two empty strings (a: "", b: "") is determined as 0.0,
/// `(a == b)`
///
/// # Examples
///
/// /// ```
/// # use str_distance::{Levenshtein, SorensenDice, TokenSet, RatcliffObershelp,
/// DistanceValue, str_distance_normalized};
/// assert_eq!(str_distance_normalized("" , "", Levenshtein::default()), 0.0);
/// assert_eq!(str_distance_normalized(" nacht", "nacht",
/// Levenshtein::default()), 0.0); assert_eq!(strdistance_normalized("abc",
/// "def", Levenshtein::default()), 1.0); ```
pub fn str_distance_normalized<S, T, D>(a: S, b: T, dist: D) -> f64
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: DistanceMetric,
{
    dist.str_normalized(a, b)
}

pub trait DistanceMetric {
    /// Represents the data type in which this distance is evaluated.
    type Dist: PartialOrd;

    /// Generic implementation of the metric.
    fn distance<S, T>(&self, _a: S, _b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        unimplemented!()
    }

    /// Evaluates the distance between two str.
    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        self.distance(a.as_ref().chars(), b.as_ref().chars())
    }

    /// Generic implementation of the normalized metric.
    fn normalized<S, T>(&self, _a: S, _b: T) -> f64
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        unimplemented!()
    }

    /// Convenience normalization for str types.
    fn str_normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        self.normalized(a.as_ref().chars(), b.as_ref().chars())
    }
}

/// Convenience trait to use a distance on a type directly.
pub trait DistanceElement {
    fn distance<S, D>(&self, other: S, dist: &D) -> <D as DistanceMetric>::Dist
    where
        S: AsRef<str>,
        D: DistanceMetric;
}

impl<T: AsRef<str>> DistanceElement for T {
    fn distance<S, D>(&self, other: S, dist: &D) -> <D as DistanceMetric>::Dist
    where
        S: AsRef<str>,
        D: DistanceMetric,
    {
        dist.str_distance(self, other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum DistanceValue {
    Exact(usize),
    Exceeded(usize),
}

impl Into<usize> for DistanceValue {
    fn into(self) -> usize {
        *self
    }
}

impl Deref for DistanceValue {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            DistanceValue::Exact(val) | DistanceValue::Exceeded(val) => val,
        }
    }
}
