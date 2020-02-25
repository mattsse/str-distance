use crate::DistanceMetric;

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

/// `TokenSort` modifies the inner string distance `dist` to adjust for
/// differences in word orders by reording words alphabetically.
///
/// For other types than strings this is just a delegate to the inner metric.
pub struct TokenSort<D: DistanceMetric> {
    /// The base distance to modify.
    inner: D,
}

impl<D> DistanceMetric for TokenSort<D>
where
    D: DistanceMetric,
{
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
        self.inner.distance(a, b)
    }

    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let mut a: Vec<_> = a.as_ref().split_whitespace().collect();
        a.sort();
        let mut b: Vec<_> = b.as_ref().split_whitespace().collect();
        b.sort();
        self.distance(a.join(" ").chars(), b.join(" ").chars())
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
