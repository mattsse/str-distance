//! Compute distances between strings

// TODO add some remarks about coefficient vs distance (= 1 - coefficient):
// in contrast to the coefficient/index of a metric, distance is a measure of dissimilarity
// see for example: https://en.wikipedia.org/wiki/Jaccard_index

#![forbid(unsafe_code)]
#![allow(unused)]

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

pub fn strdistance<S, T, D>(a: S, b: T, dist: D) -> <D as DistanceMetric>::Dist
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: DistanceMetric,
{
    unimplemented!()
}

pub trait DistanceMetric {
    /// Represents the data type in which this distance is evaluated.
    type Dist: PartialOrd;
    /// Evaluates the distance between two str.
    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>;

    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        unimplemented!()
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
        dist.distance(self, other)
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
