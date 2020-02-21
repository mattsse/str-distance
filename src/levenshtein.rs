use std::cmp::min;

use crate::utils::{delim_distinct, order_by_len_asc, DelimDistinct};
use crate::{DistanceMetric, DistanceValue};

pub struct Levenshtein;

impl DistanceMetric for Levenshtein {
    type Dist = usize;

    #[inline]
    fn distance<S, T>(&self, s1: S, s2: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        // make sure we use the shortest str for the outer loop
        let (s1, s2) = order_by_len_asc(s1.as_ref(), s2.as_ref());

        // exclude matching prefix and suffix
        let mut delim = delim_distinct(s1, s2);

        if delim.remaining_s1() == 0 {
            // the longer str starts or ends completely with the shorter str
            return delim.remaining_s2();
        }

        let mut cache: Vec<usize> = (1..=delim.remaining_s2()).collect();

        let mut result = 0;

        for (c1_idx, c1) in delim.distinct_s1.enumerate() {
            result = c1_idx + 1;
            let mut dist_c2 = c1_idx;

            for (c2_idx, c2) in delim.distinct_s2.clone().enumerate() {
                let cost = if c1 == c2 { 0usize } else { 1usize };
                let dist_c1 = dist_c2 + cost;
                dist_c2 = cache[c2_idx];
                result = min(result + 1, min(dist_c1, dist_c2 + 1));
                cache[c2_idx] = result;
            }
        }

        result
    }
}

impl Levenshtein {
    /// Normalize the metric, so that it returns always a f64 between 0 and 1
    pub fn normalized<S, T>(&self, s1: S, s2: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let s1 = s1.as_ref();
        let s2 = s2.as_ref();

        if s2.is_empty() && s1.is_empty() {
            return 1.0;
        }

        1.0 - (self.distance(s1, s2) as f64) / (s1.chars().count().max(s2.chars().count()) as f64)
    }
}

/// Specify a maximum distance. Specifying a maximum distance allows short
/// circuit exit after exceeding the maximum distance and less cell evaluation.
/// It essentially changes the time complexity from being the product of the two
/// string lengths to being just the length of the shorter string
///
/// See http://blog.softwx.net/2015/01/optimizing-damerau-levenshtein_15.html
/// Note that this is based on Sten Hjelmqvist's "Fast, memory efficient"
/// algorithm, described at http://www.codeproject.com/Articles/13525/Fast-memory-efficient-Levenshtein-algorithm.
/// This version differs by including some optimizations, and extending it to
/// the Damerau- Levenshtein algorithm.
/// Note that this is the simpler and faster optimal string alignment (aka
/// restricted edit) distance that difers slightly from the classic
/// Damerau-Levenshtein algorithm by imposing the restriction that no substring
/// is edited more than once. So for example, "CA" to "ABC" has an edit distance
/// of 2 by a complete application of Damerau-Levenshtein, but a distance of 3
/// by this method that uses the optimal string alignment algorithm. See
/// wikipedia article for more detail on this distinction.
#[derive(Clone, Default)]
pub struct DamerauLevenshtein {
    /// The maximum edit distance of interest.
    max_distance: Option<usize>,
}

impl DamerauLevenshtein {
    pub fn with_max_distance(max_distance: usize) -> Self {
        Self {
            max_distance: Some(max_distance),
        }
    }
}

impl DistanceMetric for DamerauLevenshtein {
    type Dist = DistanceValue;
    #[inline]
    fn distance<S, T>(&self, s1: S, s2: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        // make sure we use the shortest str for the outer loop
        let (s1, s2) = order_by_len_asc(s1.as_ref(), s2.as_ref());

        // exclude matching prefix prefix and suffix
        let mut delim = delim_distinct(s1, s2);

        if delim.remaining_s1() == 0 {
            // the longer str starts or ends completely with the shorter str
            return DistanceValue::Exact(delim.remaining_s2());
        }

        let len_diff = delim.remaining_s2() - delim.remaining_s1();

        if let Some(max_dist) = self.max_distance {
            if len_diff > max_dist {
                return DistanceValue::Exceeded(max_dist);
            }
        }

        let max_dist = self.max_distance.unwrap_or_else(|| delim.remaining_s2());

        let mut v0 = Vec::with_capacity(delim.remaining_s2());
        v0.extend(1..=max_dist);
        for i in max_dist + 1..delim.remaining_s2() {
            v0.push(max_dist + 1);
        }

        let remaining_s2 = delim.remaining_s2();

        // stores one level further back
        let mut v2 = vec![0usize; remaining_s2];
        let s2_offset = max_dist - (remaining_s2 - delim.remaining_s1());
        let mut s1_tmp = delim.distinct_s1.clone().next().unwrap();
        let mut s2_tmp = delim.distinct_s2.clone().next().unwrap();
        let mut s2_start = 0;
        let mut s2_end = max_dist;
        let mut current = 0;
        // whether a check for exceeding a max dist is necessary
        let have_max = max_dist < remaining_s2;

        for (s1_idx, c1) in delim.distinct_s1.enumerate() {
            let left_c1 = s1_tmp;
            s1_tmp = c1;
            let mut left = s1_idx;
            current = left + 1;
            let mut next_trans_cost = 0;

            s2_start += if s1_idx > s2_offset { 1 } else { 0 };
            s2_end += if s2_end < remaining_s2 { 1 } else { 0 };

            for (s2_idx, c2) in delim
                .distinct_s2
                .clone()
                .enumerate()
                .skip(s2_start)
                .take(s2_end - s2_start)
            {
                let above = current;
                let mut this_trans_cost = next_trans_cost;
                next_trans_cost = v2[s2_idx];
                // cost of diagonal (substitution)
                v2[s2_idx] = left;
                current = left;
                // left now equals current cost --> will be diagonal at next iteration
                left = v0[s2_idx];
                let left_c2 = s2_tmp;
                s2_tmp = c2;

                if s1_tmp != s2_tmp {
                    if left < current {
                        // insertion
                        current = left;
                    }
                    if above < current {
                        // deletion
                        current = above;
                    }
                    current += 1;
                    if (s1_idx != 0) && (s2_idx != 0) && (s1_tmp == left_c2) && (left_c1 == s2_tmp)
                    {
                        this_trans_cost += 1;
                        if this_trans_cost < current {
                            // transposition
                            current = this_trans_cost
                        };
                    }
                }
                v0[s2_idx] = current;
            }

            if have_max && (v0[s1_idx + len_diff] > max_dist) {
                return DistanceValue::Exceeded(max_dist);
            }
        }

        if current <= max_dist {
            DistanceValue::Exact(current)
        } else {
            DistanceValue::Exceeded(max_dist)
        }
    }
}

impl DamerauLevenshtein {
    pub fn normalized<S, T>(&self, s1: S, s2: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let s1 = s1.as_ref();
        let s2 = s2.as_ref();

        if s2.is_empty() && s1.is_empty() {
            return 1.0;
        }

        if let DistanceValue::Exact(val) = self.distance(s1, s2) {
            1.0 - (val as f64) / (s1.chars().count().max(s2.chars().count()) as f64)
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_dist() {
        assert_eq!(Levenshtein.distance("kitten", "sitting"), 3);
        assert_eq!(Levenshtein.distance("", ""), 0);
        assert_eq!(Levenshtein.distance("sunday", "saturday"), 3);
        assert_eq!(Levenshtein.distance("abc", ""), 3);
        let s1 = "The quick brown fox jumped over the angry dog.";
        let s2 = "Lorem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(Levenshtein.distance(s1, s2), 37);
    }

    #[test]
    fn levenshtein_normalized() {
        assert!((Levenshtein.normalized("kitten", "sitting") - 0.57142).abs() < 0.00001);
        assert!((Levenshtein.normalized("", "") - 1.0).abs() < 0.00001);
        assert!(Levenshtein.normalized("", "second").abs() < 0.00001);
        assert!(Levenshtein.normalized("first", "").abs() < 0.00001);
        assert!((Levenshtein.normalized("string", "string") - 1.0).abs() < 0.00001);
    }

    #[test]
    fn damerau_levenshtein_dist() {
        assert_eq!(*DamerauLevenshtein::default().distance("", ""), 0);
        assert_eq!(*DamerauLevenshtein::default().distance("abc", ""), 3);
        assert_eq!(*DamerauLevenshtein::default().distance("abc", "öঙ香"), 3);
        assert_eq!(
            *DamerauLevenshtein::default().distance("damerau", "aderuaxyz"),
            6
        );
        assert_eq!(
            *DamerauLevenshtein::default().distance("jellyifhs", "jellyfish"),
            2
        );
        assert_eq!(
            *DamerauLevenshtein::default().distance("cape sand recycling ", "edith ann graham"),
            17
        );
        let s1 = "The quick brown fox jumped over the angry dog.";
        let s2 = "Lehem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(*DamerauLevenshtein::default().distance(s1, s2), 36);
        assert_eq!(
            DamerauLevenshtein::with_max_distance(10).distance(s1, s2),
            DistanceValue::Exceeded(10)
        );
    }

    #[test]
    fn damerau_levenshtein_strsim() {
        let s1 = "He said he was not there yesterday; however, many people saw him there.
She had the gift of being able to paint songs.
Mrs Miller wants the entire house repainted.
Lucifer was surprised at the amount of life at Death Valley.
Three generations with six decades of life experience.";
        let s2 = "The small white buoys marked the location of hundreds of crab pots.
Everyone says they love nature until they realize how dangerous she can be.
The stranger officiates the meal.
He drank life before spitting it out.
Dan ate the clouds like cotton candy.";

        assert_eq!(
            *DamerauLevenshtein::default().distance(s1, s2),
            strsim::damerau_levenshtein(s1, s2)
        );
    }
}
