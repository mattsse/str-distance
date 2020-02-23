use crate::qgram::QGramIter;
use crate::utils::order_by_len_asc;
use crate::DistanceMetric;

/// A TokenSet distance modifies the distance of its `inner` `[Distance]` to
/// adjust for differences in word orders and word numbers by comparing the
/// intersection of two str with each str.
///
/// http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/
pub struct TokenSet<D: DistanceMetric> {
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
}

/// `Partial` metric modifies the string distance of its inner metric to return
/// the minimum distance  between the shorter string and slices of the longer
/// string
pub struct Partial<D: DistanceMetric> {
    inner: D,
}

impl<D: DistanceMetric> Partial<D> {
    /// Create a new [`Partial`] distance metric using distance `D` as base.
    pub fn new(inner: D) -> Self {
        Self { inner }
    }
}

impl<D, U> DistanceMetric for Partial<D>
where
    D: DistanceMetric<Dist = U>,
    U: Into<f64> + PartialOrd,
{
    type Dist = f64;

    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let (a, b) = order_by_len_asc(a.as_ref(), b.as_ref());

        let len_a = a.chars().count();
        let len_b = b.chars().count();

        if len_a == len_b {
            return self.inner.str_distance(a, b).into();
        }
        if len_a == 0 {
            return 1.;
        }

        let s2: Vec<_> = b.chars().collect();

        for qgram in QGramIter::new(&s2, len_a) {
            // let current = self.inner.distance(a,
            // std::str::from_utf8_unchecked(qgram.));
        }

        0.

        // s1, s2 = reorder(s1, s2)
        // len1, len2 = length(s1), length(s2)
        // len1 == len2 && return dist.dist(s1, s2, max_dist)
        // len1 == 0 && return 1.0
        // out = 1.0
        // for x in qgrams(s2, len1)
        // curr = dist.dist(s1, x, max_dist)
        // out = min(out, curr)
        // max_dist = min(out, max_dist)
        // end
        // return out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RatcliffObershelp;

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
