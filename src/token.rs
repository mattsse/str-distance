use crate::Distance;

/// A TokenSet distance modifies the distance of its `inner` `[Distance]` to
/// adjust for differences in word orders and word numbers by comparing the
/// intersection of two str with each str.
///
/// http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/
pub struct TokenSet<D: Distance> {
    inner: D,
}

impl<D: Distance> TokenSet<D> {
    /// Create a new [`TokenSet`] distance metric using distance `D` as base.
    pub fn new(inner: D) -> Self {
        Self { inner }
    }
}

impl<D: Distance> Distance for TokenSet<D> {
    type Dist = <D as Distance>::Dist;

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
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
            return self.inner.distance(a, b);
        }

        let intersect = words_intersect.join(" ");
        let a = words_a.join(" ");
        let b = words_b.join(" ");

        let dist_inter_a = self.inner.distance(&intersect, &a);
        let dist_inter_b = self.inner.distance(intersect, &b);
        let dist_a_b = self.inner.distance(a, &b);

        if dist_inter_a < dist_inter_b {
            if dist_inter_a < dist_a_b {
                dist_inter_a
            } else {
                dist_a_b
            }
        } else {
            if dist_inter_b < dist_a_b {
                dist_inter_b
            } else {
                dist_a_b
            }
        }
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
        assert_eq!(TokenSet::new(RatcliffObershelp).distance(s1, s2), 0.0);

        let s2 = "Barcelona vs Rel Madrid";
        assert_eq!(
            format!("{:.6}", TokenSet::new(RatcliffObershelp).distance(s1, s2)),
            "0.080000"
        );
    }
}
