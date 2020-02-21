use std::str::Chars;

/// Return the shorter str as first index
#[inline]
pub(crate) fn order_by_len_asc<'a>(s1: &'a str, s2: &'a str) -> (&'a str, &'a str) {
    if s1.len() <= s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    }
}

#[inline]
pub(crate) fn count_eq<Iter: Iterator<Item = char>>(mut s1_iter: Iter, mut s2_iter: Iter) -> usize {
    let mut match_ctn = 0usize;
    loop {
        let c1 = match s1_iter.next() {
            None => {
                // s2 ends with completely with s1
                break;
            }
            Some(val) => val,
        };

        let c2 = match s2_iter.next() {
            None => {
                // s1 ends completely with s2
                break;
            }
            Some(val) => val,
        };
        if c1 == c2 {
            match_ctn += 1;
        } else {
            break;
        }
    }
    match_ctn
}

/// Return the len of common prefix and suffix chars, and the distinct left
/// elements in between.
#[inline]
pub(crate) fn delim_distinct<'a>(
    s1: &'a str,
    s2: &'a str,
) -> DelimDistinct<std::iter::Skip<std::iter::Take<std::str::Chars<'a>>>> {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    let suffix_len = count_eq(s1.chars().rev(), s2.chars().rev());

    let mut s1_iter = s1.chars().take(s1_len - suffix_len);
    let mut s2_iter = s2.chars().take(s2_len - suffix_len);

    let prefix_len = count_eq(s1_iter.clone(), s2_iter.clone());

    let common_len = prefix_len + suffix_len;
    DelimDistinct {
        suffix_len,
        prefix_len,
        s1_len: s1_len - common_len,
        s2_len: s2_len - common_len,
        distinct_s1: s1_iter.skip(prefix_len),
        distinct_s2: s2_iter.skip(prefix_len),
    }
}

pub(crate) struct DelimDistinct<Iter: Iterator<Item = char>> {
    /// The amount of chars both str share at their beginning.
    pub prefix_len: usize,
    /// Iterator over the distinct chars of str s1
    pub distinct_s1: Iter,
    /// The amount of distinct chars left in str 1
    pub s1_len: usize,
    /// Iterator over the distinct chars of str s2
    pub distinct_s2: Iter,
    /// The amount of distinct chars left in str 2
    pub s2_len: usize,
    /// The amount of chars both str share at their end.
    pub suffix_len: usize,
}

impl<Iter: Iterator<Item = char>> DelimDistinct<Iter> {
    /// Amount of chars both str share at their tails.
    #[inline]
    pub fn common(&self) -> usize {
        self.prefix_len + self.suffix_len
    }

    /// The amount of distinct chars for each str
    #[inline]
    pub fn remaining(&self) -> (usize, usize) {
        (self.s1_len, self.s2_len)
    }

    /// Whether both str are identical.
    #[inline]
    pub fn is_eq(&self) -> bool {
        self.remaining() == (0, 0)
    }

    #[inline]
    pub fn remaining_s2(&self) -> usize {
        self.s2_len
    }

    #[inline]
    pub fn remaining_s1(&self) -> usize {
        self.s1_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delim_different() {
        let s1 = "kitten";
        let s2 = "sitting";

        let delim = delim_distinct(s1, s2);
        assert_eq!(delim.prefix_len, 0);
        assert_eq!(delim.suffix_len, 0);
        assert_eq!(delim.s1_len, 6);
        assert_eq!(delim.s2_len, 7);
    }

    #[test]
    fn delim_eq() {
        let s1 = "kitten";
        let s2 = "kitten";

        let delim = delim_distinct(s1, s2);
        assert_eq!(delim.common(), 6);
        assert_eq!(delim.remaining(), (0, 0));
        assert!(delim.is_eq());
    }

    #[test]
    fn delim_eq_suffix() {
        let s1 = "cute kitten";
        let s2 = "kitten";

        let delim = delim_distinct(s1, s2);
        assert_eq!(delim.common(), 6);
        assert_eq!(delim.remaining(), (5, 0));
        assert_eq!(delim.distinct_s1.collect::<String>(), String::from("cute "));

        let s1 = "k cute kitten";
        let s2 = "kitten";
        let delim = delim_distinct(s1, s2);
        assert_eq!(delim.common(), 6);
        assert_eq!(delim.remaining(), (7, 0));
        assert_eq!(
            delim.distinct_s1.collect::<String>(),
            String::from("k cute ")
        );
    }

    #[test]
    fn delim_eq_prefix() {
        let s1 = "hungry kitten";
        let s2 = "hungry hippo";

        let delim = delim_distinct(s1, s2);
        assert_eq!(delim.common(), 7);
        assert_eq!(delim.remaining(), (6, 5));
        assert_eq!(
            delim.distinct_s1.collect::<String>(),
            String::from("kitten")
        );
        assert_eq!(delim.distinct_s2.collect::<String>(), String::from("hippo"));
    }

    #[test]
    fn delim_eq_prefix_suffix() {
        let s1 = "hungry kitten is hungry";
        let s2 = "hungry hippo is hungry";

        let delim = delim_distinct(s1, s2);
        assert_eq!(delim.common(), 17);
        assert_eq!(delim.remaining(), (6, 5));
        assert_eq!(
            delim.distinct_s1.collect::<String>(),
            String::from("kitten")
        );
        assert_eq!(delim.distinct_s2.collect::<String>(), String::from("hippo"));
    }
}
