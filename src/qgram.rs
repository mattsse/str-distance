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

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        let a: Vec<_> = a.into_iter().collect();
        let b: Vec<_> = b.into_iter().collect();

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        eq_map(iter_a, iter_b)
            .into_iter()
            .map(|(n1, n2)| if n1 > n2 { n1 - n2 } else { n2 - n1 })
            .sum()
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
        let a = a.into_iter();
        let b = b.into_iter();

        let len_a = a.clone().count();
        let len_b = a.clone().count();

        if len_a.min(len_b) <= self.q {
            if a.eq(b) {
                0.
            } else {
                1.
            }
        } else {
            self.distance(a, b) as f64 / (len_a + len_b - 2 * self.q + 2) as f64
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

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        let a: Vec<_> = a.into_iter().collect();
        let b: Vec<_> = b.into_iter().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (norm_a, norm_b, norm_prod) = eq_map(iter_a, iter_b).into_iter().fold(
            (0usize, 0usize, 0usize),
            |(norm_a, norm_b, norm_prod), (n1, n2)| {
                (norm_a + n1 * n1, norm_b + n2 * n2, norm_prod + n1 * n2)
            },
        );
        1.0 - norm_prod as f64 / ((norm_a as f64).sqrt() * (norm_b as f64).sqrt())
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

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        let a: Vec<_> = a.into_iter().collect();
        let b: Vec<_> = b.into_iter().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (num_dist_a, num_dist_b, num_intersect) = count_distinct_intersect(iter_a, iter_b);

        1.0 - num_intersect as f64 / ((num_dist_a + num_dist_b) as f64 - num_intersect as f64)
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

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        let a: Vec<_> = a.into_iter().collect();
        let b: Vec<_> = b.into_iter().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (num_dist_a, num_dist_b, num_intersect) = count_distinct_intersect(iter_a, iter_b);
        1.0 - 2.0 * num_intersect as f64 / (num_dist_a + num_dist_b) as f64
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

    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: IntoIterator,
        T: IntoIterator,
        <S as IntoIterator>::IntoIter: Clone,
        <T as IntoIterator>::IntoIter: Clone,
        <S as IntoIterator>::Item: PartialEq + PartialEq<<T as IntoIterator>::Item>,
        <T as IntoIterator>::Item: PartialEq,
    {
        let a: Vec<_> = a.into_iter().collect();
        let b: Vec<_> = b.into_iter().collect();

        // edge case where an input is empty
        if a.is_empty() || b.is_empty() {
            return if a.len() == b.len() { 0. } else { 1. };
        }

        let iter_a = QGramIter::new(&a, self.q);
        let iter_b = QGramIter::new(&b, self.q);

        let (num_dist_a, num_dist_b, num_intersect) = count_distinct_intersect(iter_a, iter_b);
        1.0 - num_intersect as f64 / num_dist_a.min(num_dist_b) as f64
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
        normalized_qgram(self, self.q, a, b)
    }
}

/// A Iterator that behaves similar to [`std::slice::Chunks`], but increases the
/// start index into the slice only by one each iteration.
#[derive(Debug, Clone)]
pub struct QGramIter<'a, T> {
    items: &'a [T],
    index: usize,
    chunk_size: usize,
}

impl<'a, T> QGramIter<'a, T> {
    /// Constructs the iterator that yields all possible q-grams of the
    /// underlying slice.
    ///
    /// A chunk size greater than the length of the underlying slice will result
    /// directly in a `None` value.
    ///
    /// # Panics
    ///
    /// Panics if `q` is 0.
    pub fn new(items: &'a [T], chunk_size: usize) -> Self {
        assert_ne!(chunk_size, 0);
        Self {
            items,
            chunk_size,
            index: 0,
        }
    }

    /// Resets the index back the beginning.
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<'a, T> Iterator for QGramIter<'a, T> {
    type Item = &'a [T];

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

    if len_a.min(len_b) <= q {
        if a.eq(b) {
            0.
        } else {
            1.
        }
    } else {
        metric.distance(a, b)
    }
}

fn count_distinct_intersect<S, T>(a: QGramIter<S>, b: QGramIter<T>) -> (usize, usize, usize)
where
    S: PartialEq + PartialEq<T>,
    T: PartialEq,
{
    eq_map(a, b).into_iter().fold(
        (0, 0, 0),
        |(num_dist_a, num_dist_b, num_intersect), (n1, n2)| {
            if n1 > 0 {
                if n2 > 0 {
                    (num_dist_a + 1, num_dist_b + 1, num_intersect + 1)
                } else {
                    (num_dist_a + 1, num_dist_b, num_intersect)
                }
            } else if n2 > 0 {
                (num_dist_a, num_dist_b + 1, num_intersect)
            } else {
                (num_dist_a, num_dist_b, num_intersect)
            }
        },
    )
}

/// Returns a list of tuples with the numbers of times a qgram appears in a and
/// b
///
/// This exists only to remove the necessity for `S: Hash + Eq, T:Hash + Eq`.
fn eq_map<'a, S, T>(a: QGramIter<'a, S>, b: QGramIter<'a, T>) -> Vec<(usize, usize)>
where
    S: PartialEq + PartialEq<T>,
    T: PartialEq,
{
    // remove duplicates and count how often a qgram occurs
    fn count_distinct<U: PartialEq>(v: &mut Vec<(U, usize)>) {
        'outer: for idx in (0..v.len()).rev() {
            let (qgram, num) = v.swap_remove(idx);
            for (other, num_other) in v.iter_mut() {
                if *other == qgram {
                    *num_other += num;
                    continue 'outer;
                }
            }
            v.push((qgram, num));
        }
    }
    let mut distinct_a: Vec<_> = a.map(|s| (s, 1usize)).collect();
    let mut distinct_b: Vec<_> = b.map(|s| (s, 1usize)).collect();

    count_distinct(&mut distinct_a);
    count_distinct(&mut distinct_b);

    let mut nums: Vec<_> = distinct_a.iter().map(|(_, n)| (*n, 0usize)).collect();

    'outer: for (qgram_b, num_b) in distinct_b {
        for (idx, (qgram_a, num_a)) in distinct_a.iter().enumerate() {
            if *qgram_a == qgram_b {
                nums[idx] = (*num_a, num_b);
                continue 'outer;
            }
        }
        nums.push((0, num_b));
    }
    nums
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
            format!(
                "{:.6}",
                SorensenDice::new(1).str_distance("monday", "montag")
            ),
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
        let iter = QGramIter::new(&s, 2);
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.count(), 4);

        let s: Vec<_> = "hello".chars().collect();
        let iter = QGramIter::new(&s, 3);
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

    #[test]
    fn count_eq() {
        let s1: Vec<_> = "abc".chars().collect();
        let s2: Vec<_> = "abc".chars().collect();
        let q1 = QGramIter::new(&s1, 2);
        let q2 = QGramIter::new(&s2, 2);

        assert_eq!(eq_map(q1, q2), vec![(1, 1), (1, 1)]);

        let s1: Vec<_> = "abc".chars().collect();
        let s2: Vec<_> = "abcdef".chars().collect();
        let q1 = QGramIter::new(&s1, 2);
        let q2 = QGramIter::new(&s2, 2);

        assert_eq!(eq_map(q1, q2), vec![(1, 1), (1, 1), (0, 1), (0, 1), (0, 1)]);
    }
}
