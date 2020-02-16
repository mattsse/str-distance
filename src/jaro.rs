use crate::utils::{count_eq, order_by_len_asc};
use crate::DistanceValue;

pub struct Jaro;

impl Jaro {
    #[inline]
    pub fn distance<S, T>(&self, s1: S, s2: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let (s1, s2) = order_by_len_asc(s1.as_ref(), s2.as_ref());

        let s1_len = s1.chars().count();
        let s2_len = s2.chars().count();

        // edge cases
        if s2_len == 0 {
            // guaranteed s1 <= s2 --> both are empty
            return 0.0;
        }
        if s1_len == 0 {
            return 1.0;
        }
        if s2_len == 1 {
            return if s1.chars().next().unwrap().eq(&s2.chars().next().unwrap()) {
                0.0
            } else {
                1.0
            };
        }

        let max_dist = ((s2_len / 2) - 1).max(0);

        let mut flags = vec![false; s2_len];

        let mut transpositions = 0.0;
        let mut c2_match_idx = 0;

        let mut matches = 0.0;

        for (s1_idx, c1) in s1.chars().enumerate() {
            let min_bound =
                // prevent integer wrapping
                if s1_idx > max_dist {
                    0.max( s1_idx - max_dist)
                } else {
                    0
                };

            let max_bound = (s2_len - 1).min(s1_idx + max_dist);

            if min_bound > max_bound {
                continue;
            }

            for (s2_idx, c2) in s2.chars().enumerate() {
                if min_bound <= s2_idx && s2_idx <= max_bound && c1 == c2 && !flags[s2_idx] {
                    flags[s2_idx] = true;
                    matches += 1.0;

                    if s2_idx < c2_match_idx {
                        transpositions += 1.0;
                    }
                    c2_match_idx = s2_idx;
                    break;
                }
            }
        }

        if matches == 0.0 {
            0.0
        } else {
            (1.0 / 3.0)
                * ((matches / s1_len as f64)
                    + (matches / s2_len as f64)
                    + ((matches - transpositions) / matches))
        }
    }
}

pub struct JaroWinkler;

impl JaroWinkler {
    #[inline]
    pub fn distance<S, T>(&self, s1: S, s2: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let s1 = s1.as_ref();
        let s2 = s2.as_ref();
        let mut dist = Jaro.distance(s1, s2);

        let prefix_ctn = count_eq(s1.chars(), s2.chars());

        dist += (0.1 * prefix_ctn as f64 * (1.0 - dist));

        if dist <= 1.0 {
            0.0
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// https://github.com/apache/commons-text/blob/master/src/main/java/org/apache/commons/text/similarity/JaroWinklerDistance.java
    #[test]
    fn jaro_winkler() {
        assert_eq!(JaroWinkler.distance("", ""), 0.0);
        assert_eq!(JaroWinkler.distance("foo", "foo"), 0.0);
        //        assert_eq!(JaroWinkler.distance("foo", "foo "), 0.06);
        //        assert_eq!(JaroWinkler.distance("foo", "foo  "), 0.09);
        //        assert_eq!(JaroWinkler.distance("foo", " foo "), 0.13);
        //        assert_eq!(JaroWinkler.distance("foo", "  foo"), 0.49);
        //        assert_eq!(JaroWinkler.distance("", "a"), 1.0);
        //        assert_eq!(JaroWinkler.distance("aaapppp", ""), 1.0);
        //        assert_eq!(JaroWinkler.distance("frog", "fog"), 0.07);
        //        assert_eq!(JaroWinkler.distance("fly", "ant"), 1.0);
        //        assert_eq!(JaroWinkler.distance("elephant", "hippo"), 0.56);
        //        assert_eq!(JaroWinkler.distance("hippo", "elephant"), 0.56);
        //        assert_eq!(JaroWinkler.distance("hippo", "zzzzzzzz"), 1.0);
        //        assert_eq!(JaroWinkler.distance("hello", "hallo"), 0.12);
        //        assert_eq!(JaroWinkler.distance("ABC Corporation", "ABC
        // Corp"), 0.09);        assert_eq!(
        //            JaroWinkler.distance("D N H Enterprises Inc", "D &amp; H
        // Enterprises, Inc."),            0.05
        //        );
        //        assert_eq!(
        //            JaroWinkler.distance(
        //                "My Gym Children's Fitness Center",
        //                "My Gym. Childrens); Fitness"
        //            ),
        //            0.08
        //        );
        //        assert_eq!(JaroWinkler.distance("PENNSYLVANIA",
        // "PENNCISYLVNIA"), 0.12);
    }
}
