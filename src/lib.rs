//! Compute distances between strings

#![allow(unused)]
pub mod distances;

pub mod jaro;
pub mod levenshtein;
mod utils;

pub use levenshtein::{DamerauLevenshtein, Levenshtein};
use std::ops::Deref;

pub fn strdistance<S, T, D>(s1: S, s2: T, dist: D)
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: Distance,
{
}

pub fn strcompare<S, T, D>(s1: S, s2: T, dist: D)
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: Distance,
{
}

// drawbacks from trait: needs instance

pub trait Distance {
    // TODO make return type an associated type:
    // type Dist;
    fn distance<S, T>(&self, s1: S, s2: T) -> usize
    where
        S: AsRef<str>,
        T: AsRef<str>;

    fn compare<S, T, U>(&self, s1: S, s2: T, min_score: U)
    where
        S: AsRef<str>,
        T: AsRef<str>,
        U: Into<Score>,
    {
    }
}

pub struct Score(pub f64);

/// Convenience
pub trait DistanceElement {
    fn distance<S, D>(&self, other: S, dist: D)
    where
        S: AsRef<str>,
        D: Distance;
}

impl<T: AsRef<str>> DistanceElement for T {
    fn distance<S, D>(&self, other: S, dist: D)
    where
        S: AsRef<str>,
        D: Distance,
    {
        let s1 = self.as_ref();

        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
