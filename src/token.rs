use crate::qgram::QGramIter;
use crate::utils::order_by_len_asc;
use crate::{DistanceMetric, RatcliffObershelp};
use std::cmp;

/// A TokenSet distance modifies the distance of its `inner` `[Distance]` to
/// adjust for differences in word orders and word numbers by comparing the
/// intersection of two str with each str.
///
/// http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/
pub struct TokenSet<D: DistanceMetric> {
    /// The base distance to modify.
    inner: D,
}

impl<D: DistanceMetric> TokenSet<D> {
    /// Create a new [`TokenSet`] distance metric using distance `D` as base.
    pub fn new(inner: D) -> Self {
        Self { inner }
    }
}

impl<D: DistanceMetric> DistanceMetric for TokenSet<D> {
    type Dist = <D as DistanceMetric>::Dist;

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

        let intersect = b.clone().filter(|x| a.clone().any(|y| y == *x));

        if intersect.clone().count() == 0 {
            return self.inner.distance(a, b);
        }

        let dist_inter_a = self.inner.distance(a.clone(), intersect.clone());
        let dist_inter_b = self.inner.distance(intersect, b.clone());
        let dist_a_b = self.inner.distance(a, b);

        if dist_inter_a < dist_inter_b {
            if dist_inter_a < dist_a_b {
                dist_inter_a
            } else {
                dist_a_b
            }
        } else if dist_inter_b < dist_a_b {
            dist_inter_b
        } else {
            dist_a_b
        }
    }

    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let a = a.as_ref();
        let mut words_a: Vec<_> = a.split_whitespace().collect();
        words_a.sort();
        words_a.dedup_by(|a, b| a == b);

        let b = b.as_ref();
        let mut words_b: Vec<_> = b.split_whitespace().collect();
        words_b.sort();
        words_b.dedup_by(|a, b| a == b);

        let words_intersect: Vec<_> = words_b
            .iter()
            .cloned()
            .filter(|s| words_a.contains(s))
            .collect();

        if words_intersect.is_empty() {
            return self.inner.str_distance(a, b);
        }

        let intersect = words_intersect.join(" ");
        let a = words_a.join(" ");
        let b = words_b.join(" ");

        let dist_inter_a = self.inner.str_distance(&intersect, &a);
        let dist_inter_b = self.inner.str_distance(intersect, &b);
        let dist_a_b = self.inner.str_distance(a, &b);

        if dist_inter_a < dist_inter_b {
            if dist_inter_a < dist_a_b {
                dist_inter_a
            } else {
                dist_a_b
            }
        } else if dist_inter_b < dist_a_b {
            dist_inter_b
        } else {
            dist_a_b
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
        self.inner.normalized(a, b)
    }
}

/// `Partial` metric modifies the string distance of its inner metric to return
/// the minimum distance  between the shorter string and slices of the longer
/// string
pub struct Partial<D: DistanceMetric> {
    /// The base distance to modify.
    inner: D,
}

impl<D: DistanceMetric> Partial<D> {
    /// Create a new [`Partial`] distance metric using distance `D` as base.
    pub fn new(inner: D) -> Self {
        Self { inner }
    }
}

impl<D> DistanceMetric for Partial<D>
where
    D: DistanceMetric,
    <D as DistanceMetric>::Dist: Into<f64>,
{
    type Dist = f64;

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
        let a = a.into_iter();
        let b = b.into_iter();

        let len_a = a.clone().count();
        let len_b = b.clone().count();

        if len_a == len_b {
            return self.inner.distance(a.clone(), b.clone()).into();
        }
        if cmp::min(len_a, len_b) == 0 {
            return 1.;
        }

        let s2: Vec<_> = b.clone().collect();
        let mut out = 1.;

        for qgram in QGramIter::new(&s2, len_a) {
            let current = self.inner.distance(a.clone(), b.clone()).into();
            out = if out < current { out } else { current };
        }

        out
    }

    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let (a, b) = order_by_len_asc(a.as_ref(), b.as_ref());
        self.distance(a.chars(), b.chars())
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
    use crate::RatcliffObershelp;

    use super::*;

    #[test]
    fn token_set_ratcliff() {
        let s1 = "Real Madrid vs FC Barcelona";
        let s2 = "Barcelona vs Real Madrid";
        assert_eq!(TokenSet::new(RatcliffObershelp).str_distance(s1, s2), 0.0);

        let s2 = "Barcelona vs Rel Madrid";
        assert_eq!(
            format!(
                "{:.6}",
                TokenSet::new(RatcliffObershelp).str_distance(s1, s2)
            ),
            "0.080000"
        );
    }
}
