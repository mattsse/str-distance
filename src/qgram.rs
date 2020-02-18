use std::slice::Chunks;

/// Represents the length of q-gram
pub struct Qgram(pub usize);

/// Return an iterator on the q-gram of a string
fn qgrams<Iter: IntoIterator>(iter: Iter, q: usize) -> QgramIter<Iter::IntoIter> {
    QgramIter::new(iter.into_iter(), q)
}

/// A Iterator that behaves similar to [`std::slice::Chunks`].
struct QIter<'a> {
    elements: &'a [char],
    index: usize,
}

impl<'a> Iterator for QIter<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

/// TODO first draft from https://stackoverflow.com/questions/42134874/are-there-equivalents-to-slicechunks-windows-for-iterators-to-loop-over-pairs
/// The desired QgramIter should probably store a slice of char and yield a
/// sliding window
pub struct QgramIter<I: Iterator> {
    chunks: Vec<<I as Iterator>::Item>,
    underlying: I,
}

impl<I: Iterator> QgramIter<I> {
    fn new(iter: I, size: usize) -> Self {
        let mut result = Self {
            underlying: iter,
            chunks: Vec::with_capacity(size),
        };
        result.refill(size);
        result
    }

    fn refill(&mut self, size: usize) {
        for _ in 0..size {
            match self.underlying.next() {
                Some(item) => self.chunks.push(item),
                None => break,
            }
        }
    }
}

impl<I: Iterator> Iterator for QgramIter<I> {
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunks.is_empty() {
            return None;
        }
        let next_chunks = Vec::with_capacity(self.chunks.len());
        let result = std::mem::replace(&mut self.chunks, next_chunks);

        self.refill(result.len());

        Some(result)
    }
}

/// Cosine metric
pub struct Cosine(pub usize);
