use crate::Distance;
use std::collections::HashMap;
use std::slice::Chunks;

/// Represents a QGram metric where `q` is the length of a q-gram fragment.
///
/// The distance corresponds to
///
///  `||v(s1, q) - v(s2, q)||`
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

impl Distance for QGram {
    fn distance<S, T>(&self, a: S, b: T) -> usize
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
struct QGramIter<'a> {
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

/// Cosine metric
pub struct Cosine(pub usize);

#[cfg(test)]
mod tests {
    use super::*;

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
