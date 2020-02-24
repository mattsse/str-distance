//! Compute distances between strings types (and others)
//!
//! This crate provides implementations for a variety of distance or equality
//! metrics. When using metrics that are a measure of **similarity**, the
//! following should be noted: All implementations always return the value of
//! the distance between two elements (e.g. str), i.e. their degree of
//! **dissimilarity**. Which the implemented metrics that are designed to measure similarity (e.g. [Jaccard index](https://en.wikipedia.org/wiki/Jaccard_index)) will return the distance, which is complementary to the similarity score.
//!
//! # Usage
//!
//! ## The `str_distance::str_distance*` convenience functions.
//!
//! `str_distance` and `str_distance_normalized` take the two string inputs for
//! which the distance is determined using the passed 'DistanceMetric`.
//! `str_distance_normalized` evaluates the normalized distance between two
//! strings. A value of '0.0' corresponds to the "zero distance", both strings
//! are considered equal by means of the metric, whereas a value of '1.0'
//! corresponds to the maximum distance that can exist between the strings.
//!
//! Calling the `str_distance::str_distance*` is just convenience for
//! `DistanceMetric.str_distance*("", "")`
//!
//! ### Example
//!
//! Levenshtein metrics offer the possibility to define a maximum distance at
//! which the further calculation of the exact distance is aborted early.
//!
//! **Distance**
//!
//! ```rust
//! use str_distance::*;
//!
//! // calculate the exact distance
//! assert_eq!(str_distance("kitten", "sitting", Levenshtein::default()), DistanceValue::Exact(3));
//!
//! // short circuit if distance exceeds 10
//! let s1 = "Wisdom is easily acquired when hiding under the bed with a saucepan on your head.";
//! let s2 = "The quick brown fox jumped over the angry dog.";
//! assert_eq!(str_distance(s1, s2, Levenshtein::with_max_distance(10)), DistanceValue::Exceeded(10));
//! ```
//!
//! **Normalized Distance**
//!
//! ```rust
//! use str_distance::*;
//! assert_eq!(str_distance_normalized("" , "", Levenshtein::default()), 0.0);
//! assert_eq!(str_distance_normalized("nacht", "nacht", Levenshtein::default()), 0.0);
//! assert_eq!(str_distance_normalized("abc", "def", Levenshtein::default()), 1.0);
//! ```
//!
//! ## The `DistanceMetric` trait
//!
//! ```rust
//! use str_distance::{DistanceMetric, SorensenDice};
//! // QGram metrics require the length of the underlying fragment length to use for comparison.
//! // For `SorensenDice` default is 2.
//! assert_eq!(SorensenDice::new(2).str_distance("nacht", "night"), 0.75);
//! ```
//!
//! `DistanceMetric` was designed for `str` types, but is not limited to.
//! Calculating distance is possible for all data types which are comparable and
//! are passed as 'IntoIterator', e.g. as `Vec` or slice
//!
//! ```rust
//! use str_distance::{DistanceMetric, Levenshtein, DistanceValue};
//!
//! assert_eq!(*Levenshtein::default().distance(&[1,2,3], &[1,2,3,4,5,6]),3);
//! ```

#![forbid(unsafe_code)]
#![allow(unused)]

use std::ops::Deref;

pub use jaro::{Jaro, Winkler};
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
/// assert_eq!(str_distance_normalized("nacht", "nacht",
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
    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
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

    /// Evaluates the normalized distance between two strings
    /// A value of '0.0' corresponds to the "zero distance", both strings are
    /// considered equal by means of the metric, whereas a value of '1.0'
    /// corresponds to the maximum distance that can exist between the strings.
    fn normalized<S, T>(&self, a: S, b: T) -> f64
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
