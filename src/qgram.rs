use std::collections::HashMap;

use crate::utils::order_by_len_asc;
use crate::DistanceMetric;

/// Represents a QGram metric where `q` is the length of a q-gram fragment.
///
/// The distance corresponds to
///
/// ```text
///     ||v(s1, q) - v(s2, q)||
/// ```
///
/// where `v(s, q)` denotes the vec on the space of q-grams of length q,
/// that contains the number of times a q-gram fragment appears for the str s
#[derive(Debug, Clone)]
pub struct QGram {
    /// Length of the fragment
    q: usize,
}

impl QGram {
    /// Creates a new [`QGram]` of length `q`.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(q: usize) -> Self {
        assert_ne!(q, 0);
        Self { q }
    }
}

impl DistanceMetric for QGram {
    type Dist = usize;
    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let a: Vec<_> = a.as_ref().chars().collect();
        let b: Vec<_> = b.as_ref().chars().collect();

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        eq_map(iter_a, iter_b)
            .values()
            .cloned()
            .map(|(n1, n2)| if n1 > n2 { n1 - n2 } else { n2 - n1 })
            .sum()
    }

    /// Normalize the metric, so that it returns always a f64 between 0 and 1.
    /// If a str length < q, returns a == b
    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let (a, b) = order_by_len_asc(a.as_ref(), b.as_ref());

        let len_a = a.chars().count();
        if len_a <= self.q {
            if a == b {
                0.
            } else {
                1.
            }
        } else {
            let len_b = b.chars().count();
            self.str_distance(a, b) as f64 / (len_a + len_b - 2 * self.q + 2) as f64
        }
    }
}

/// The Cosine distance corresponds to
///
/// ```text
///     1 - v(s1, q).v(s2, q)  / ||v(s1, q)|| * ||v(s2, q)||
/// ```
///
/// where `v(s, q)` denotes the vec on the space of q-grams of length q,
/// that contains the  number of times a q-gram appears for the str s.
///
/// If both inputs are empty a value of `0.` is returned. If one input is empty
/// and the other is not, a value of `1.` is returned. This avoids a return of
/// `f64::NaN` for those cases.
#[derive(Debug, Clone)]
pub struct Cosine {
    /// Length of the fragment
    q: usize,
}

impl Cosine {
    /// Creates a new [`Cosine]` metric of length `q`.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(q: usize) -> Self {
        assert_ne!(q, 0);
        Self { q }
    }
}

impl DistanceMetric for Cosine {
    type Dist = f64;
    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let a: Vec<_> = a.as_ref().chars().collect();
        let b: Vec<_> = b.as_ref().chars().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (norm_a, norm_b, norm_prod) = eq_map(iter_a, iter_b).values().cloned().fold(
            (0usize, 0usize, 0usize),
            |(norm_a, norm_b, norm_prod), (n1, n2)| {
                (norm_a + n1 * n1, norm_b + n2 * n2, norm_prod + n1 * n2)
            },
        );
        1.0 - norm_prod as f64 / ((norm_a as f64).sqrt() * (norm_b as f64).sqrt())
    }

    /// Normalize the metric, so that it returns always a f64 between 0 and 1.
    /// If a str length < q, returns a == b
    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        normalized_qgram(self, self.q, a, b)
    }
}

/// Represents a Jaccard metric where `q` is the length of a q-gram fragment.
///
/// The distance corresponds to
///
/// ```text
///     1 - |Q(s1, q) ∩ Q(s2, q)| / |Q(s1, q) ∪ Q(s2, q))|
/// ```
///
/// where ``Q(s, q)``  denotes the set of q-grams of length n for the str s.
///
/// If both inputs are empty a value of `0.` is returned. If one input is empty
/// and the other is not, a value of `1.` is returned. This avoids a return of
/// `f64::NaN` for those cases.
#[derive(Debug, Clone)]
pub struct Jaccard {
    /// Length of the fragment
    q: usize,
}

impl Jaccard {
    /// Creates a new [`Jaccard]` of length `q`.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(q: usize) -> Self {
        assert_ne!(q, 0);
        Self { q }
    }
}

impl DistanceMetric for Jaccard {
    type Dist = f64;
    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let a: Vec<_> = a.as_ref().chars().collect();
        let b: Vec<_> = b.as_ref().chars().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (num_dist_a, num_dist_b, num_intersect) = count_distinct_intersect(iter_a, iter_b);

        1.0 - num_intersect as f64 / ((num_dist_a + num_dist_b) as f64 - num_intersect as f64)
    }

    /// Normalize the metric, so that it returns always a f64 between 0 and 1.
    /// If a str length < q, returns a == b
    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        normalized_qgram(self, self.q, a, b)
    }
}

/// Represents a SorensenDice metric where `q` is the length of a q-gram
/// fragment.
///
/// The distance corresponds to
///
/// ```text
///     1 - 2 * |Q(s1, q) ∩ Q(s2, q)|  / (|Q(s1, q)| + |Q(s2, q))|)
/// ```
///
/// where `Q(s, q)`  denotes the set of q-grams of length n for the str s
///
/// If both inputs are empty a value of `1.` is returned. If one input is empty
/// and the other is not, a value of `0.` is returned. This avoids a return of
/// `f64::NaN` for those cases.
#[derive(Debug, Clone)]
pub struct SorensenDice {
    /// Length of the fragment
    q: usize,
}

impl SorensenDice {
    /// Creates a new [`SorensenDice]` of length `q`.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(q: usize) -> Self {
        assert_ne!(q, 0);
        Self { q }
    }
}

impl Default for SorensenDice {
    /// Use a bigram as default fragment length.
    fn default() -> Self {
        SorensenDice::new(2)
    }
}

impl DistanceMetric for SorensenDice {
    type Dist = f64;
    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let a: Vec<_> = a.as_ref().chars().collect();
        let b: Vec<_> = b.as_ref().chars().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (num_dist_a, num_dist_b, num_intersect) = count_distinct_intersect(iter_a, iter_b);
        1.0 - 2.0 * num_intersect as f64 / (num_dist_a + num_dist_b) as f64
    }

    /// Normalize the metric, so that it returns always a f64 between 0 and 1.
    /// If a str length < q, returns a == b
    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        normalized_qgram(self, self.q, a, b)
    }
}

/// Represents a Overlap metric where `q` is the length of a q-gram
/// fragment.
///
/// The distance corresponds to
///
/// ```text
///     1 - |Q(s1, q) ∩ Q(s2, q)|  / min(|Q(s1, q)|, |Q(s2, q)|)
/// ```
///
/// where `Q(s, q)`  denotes the set of q-grams of length n for the str s
///
/// If both inputs are empty a value of `1.` is returned. If one input is empty
/// and the other is not, a value of `0.` is returned. This avoids a return of
/// `f64::NaN` for those cases.
#[derive(Debug, Clone)]
pub struct Overlap {
    /// Length of the fragment
    q: usize,
}

impl Overlap {
    /// Creates a new [`Overlap]` of length `q`.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(q: usize) -> Self {
        assert_ne!(q, 0);
        Self { q }
    }
}

impl Default for Overlap {
    /// Use a monogram as default overlap fragment length.
    fn default() -> Self {
        Overlap::new(1)
    }
}

impl DistanceMetric for Overlap {
    type Dist = f64;
    fn str_distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let a: Vec<_> = a.as_ref().chars().collect();
        let b: Vec<_> = b.as_ref().chars().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (num_dist_a, num_dist_b, num_intersect) = count_distinct_intersect(iter_a, iter_b);
        1.0 - num_intersect as f64 / num_dist_a.min(num_dist_b) as f64
    }

    /// Normalize the metric, so that it returns always a f64 between 0 and 1.
    /// If a str length < q, returns a == b
    fn normalized<S, T>(&self, a: S, b: T) -> f64
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        normalized_qgram(self, self.q, a, b)
    }
}

fn eq_map<'a>(mut a: QGramIter<'a>, mut b: QGramIter<'a>) -> HashMap<&'a [char], (usize, usize)> {
    let mut set = HashMap::new();

    for qa in a {
        let (x, _) = set.entry(qa).or_insert((0, 0));
        *x += 1;
    }
    for qb in b {
        let (_, y) = set.entry(qb).or_insert((0, 0));
        *y += 1;
    }

    set
}

/// A Iterator that behaves similar to [`std::slice::Chunks`], but increases the
/// start index into the slice only by one each iteration.
pub(crate) struct QGramIter<'a> {
    items: &'a [char],
    index: usize,
    chunk_size: usize,
}

impl<'a> QGramIter<'a> {
    /// Constructs the iterator that yields all possible q-grams of the
    /// underlying slice.
    ///
    /// A chunk size greater than the length of the underlying slice will result
    /// directly in a `None` value.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(items: &'a [char], chunk_size: usize) -> Self {
        assert_ne!(chunk_size, 0);
        Self {
            items,
            chunk_size,
            index: 0,
        }
    }
}

impl<'a> Iterator for QGramIter<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty()
            || self.index + self.chunk_size > self.items.len()
            || self.chunk_size > self.items.len()
        {
            None
        } else {
            let q = &self.items[self.index..self.index + self.chunk_size];
            self.index += 1;
            Some(q)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.items.is_empty()
            || self.index + self.chunk_size > self.items.len()
            || self.chunk_size > self.items.len()
        {
            (0, Some(0))
        } else {
            let n = self.items.len() + 1 - self.chunk_size;
            let rem = n - self.index;
            (rem, Some(rem))
        }
    }
}

/// Normalize the metric, so that it returns always a f64 between 0 and 1.
/// If a str length < q, returns a == b
fn normalized_qgram<Q, S, T>(metric: &Q, q: usize, a: S, b: T) -> Q::Dist
where
    Q: DistanceMetric<Dist = f64>,
    S: AsRef<str>,
    T: AsRef<str>,
{
    let (a, b) = order_by_len_asc(a.as_ref(), b.as_ref());

    let len_a = a.chars().count();
    if len_a <= q {
        if a == b {
            0.
        } else {
            1.
        }
    } else {
        metric.str_distance(a, b)
    }
}

fn count_distinct_intersect(a: QGramIter, b: QGramIter) -> (usize, usize, usize) {
    eq_map(a, b).values().cloned().fold(
        (0, 0, 0),
        |(num_dist_a, num_dist_b, num_intersect), (n1, n2)| {
            if n1 > 0 {
                if n2 > 0 {
                    (num_dist_a + 1, num_dist_b + 1, num_intersect + 1)
                } else {
                    (num_dist_a + 1, num_dist_b, num_intersect)
                }
            } else {
                if n2 > 0 {
                    (num_dist_a, num_dist_b + 1, num_intersect)
                } else {
                    (num_dist_a, num_dist_b, num_intersect)
                }
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qgram_distance() {
        assert_eq!(QGram::new(2).str_distance("abc", "abc"), 0);
        assert_eq!(QGram::new(1).str_distance("abc", "cba"), 0);
        assert_eq!(QGram::new(1).str_distance("abc", "ccc"), 4);
        assert_eq!(QGram::new(4).str_distance("aü☃", "aüaüafs"), 4);
        assert_eq!(QGram::new(4).str_distance("abcdefg", "defgabc"), 6);
    }

    #[test]
    fn cosine_distance() {
        assert_eq!(Cosine::new(1).str_distance("", ""), 0.);
        assert_eq!(Cosine::new(2).str_distance("abc", "ccc"), 1.);
        assert_eq!(
            format!("{:.6}", Cosine::new(2).str_distance("leia", "leela")),
            "0.711325"
        );
        assert_eq!(
            format!("{:.6}", Cosine::new(2).str_distance("achieve", "acheive")),
            "0.500000"
        );
        assert_eq!(Cosine::new(3).str_distance("achieve", "acheive"), 0.8);
    }

    #[test]
    fn jaccard_distance() {
        assert_eq!(Jaccard::new(1).str_distance("", ""), 0.);
        assert_eq!(Jaccard::new(1).str_distance("", "x"), 1.);
        assert_eq!(Jaccard::new(3).str_distance("abc", "abc"), 0.);
        assert_eq!(
            format!("{:.6}", Jaccard::new(1).str_distance("abc", "ccc")),
            "0.666667"
        );
    }

    #[test]
    fn sorensen_dice_distance() {
        assert_eq!(SorensenDice::new(1).str_distance("", ""), 0.);
        assert_eq!(SorensenDice::new(3).str_distance("", "abc"), 1.);
        assert_eq!(SorensenDice::new(3).str_distance("abc", "abc"), 0.);
        assert_eq!(SorensenDice::new(3).str_distance("abc", "xxx"), 1.);
        assert_eq!(SorensenDice::new(2).str_distance("monday", "montag"), 0.6);
        assert_eq!(SorensenDice::new(2).str_distance("nacht", "night"), 0.75);

        // 1-
        // assert_eq!(SorensenDice::new(2).distance("nacht", "night"),
        // strsim::sorensen_dice("nacht", "night"))
    }

    #[test]
    fn overlap_distance() {
        assert_eq!(SorensenDice::new(1).str_distance("", ""), 0.);
        assert_eq!(SorensenDice::new(1).str_distance("", "abc"), 1.);
        assert_eq!(SorensenDice::new(3).str_distance("abc", "abc"), 0.);
        assert_eq!(SorensenDice::new(3).str_distance("abc", "xxx"), 1.);
        assert_eq!(
            format!("{:.6}", SorensenDice::new(1).str_distance("monday", "montag")),
            "0.333333"
        );
        assert_eq!(SorensenDice::new(1).str_distance("nacht", "night"), 0.4);
    }

    #[test]
    fn qgram_iter() {
        let s: Vec<_> = "hello".chars().collect();
        let mut iter = QGramIter::new(&s, 2);
        assert_eq!(iter.next(), Some(['h', 'e'].as_ref()));
        assert_eq!(iter.next(), Some(['e', 'l'].as_ref()));
        assert_eq!(iter.next(), Some(['l', 'l'].as_ref()));
        assert_eq!(iter.next(), Some(['l', 'o'].as_ref()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let s: Vec<_> = "abc".chars().collect();
        let mut iter = QGramIter::new(&s, 2);
        assert_eq!(iter.next(), Some(['a', 'b'].as_ref()));
        assert_eq!(iter.next(), Some(['b', 'c'].as_ref()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty_qgram() {
        let s: Vec<_> = "".chars().collect();
        let mut iter = QGramIter::new(&s, 1);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn single_qgram() {
        let s: Vec<_> = "hel".chars().collect();
        let mut iter = QGramIter::new(&s, 3);
        assert_eq!(iter.next(), Some(['h', 'e', 'l'].as_ref()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn exceeding_qgram() {
        let s: Vec<_> = "hel".chars().collect();
        let mut iter = QGramIter::new(&s, 4);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn size_hint() {
        let s: Vec<_> = "hel".chars().collect();
        let iter = QGramIter::new(&s, 4);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let s: Vec<_> = "hello".chars().collect();
        let mut iter = QGramIter::new(&s, 2);
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.count(), 4);

        let s: Vec<_> = "hello".chars().collect();
        let mut iter = QGramIter::new(&s, 3);
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.count(), 3);

        let s: Vec<_> = "hello00000".chars().collect();
        let mut iter = QGramIter::new(&s, 5);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        iter.next();
        assert_eq!(iter.size_hint(), (5, Some(5)));

        let s: Vec<_> = "hello00000".chars().collect();
        let mut iter = QGramIter::new(&s, 1);
        assert_eq!(iter.size_hint(), (10, Some(10)));
        iter.nth(8);
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}
