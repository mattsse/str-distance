str-distance
=====================
[![Build Status](https://travis-ci.com/mattsse/str-distance.svg?branch=master)](https://travis-ci.com/mattsse/str-distance)
[![Crates.io](https://img.shields.io/crates/v/str-distance.svg)](https://crates.io/crates/str-distance)
[![Documentation](https://docs.rs/str-distance/badge.svg)](https://docs.rs/str-distance)

A crate to evaluate distances between strings (and others).

Heavily inspired by the julia [StringDistances](https://github.com/matthieugomez/StringDistances.jl)

## Distance Metrics

- [Jaro Distance](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
- [Levenshtein Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
- [Damerau-Levenshtein Distance](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance) 
- [RatcliffObershelp Distance](https://xlinux.nist.gov/dads/HTML/ratcliffObershelp.html)

- Q-gram distances compare the set of all slices of length `q` in each str, where `q > 0`
	- QGram Distance `Qgram::new(usize)`
	- [Cosine Distance](https://en.wikipedia.org/wiki/Cosine_similarity) `Cosine::new(usize)`
	- [Jaccard Distance](https://en.wikipedia.org/wiki/Jaccard_index) `Jaccard::new(usize)`
	- [Sorensen-Dice Distance](https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient) `SorensenDice::new(usize)`
	- [Overlap Distance](https://en.wikipedia.org/wiki/Overlap_coefficient) `Overlap::new(usize)`
	
- The crate includes distance "modifiers", that can be applied to any distance.
	- [Winkler](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance) diminishes the distance of strings with common prefixes. The Winkler adjustment was originally defined for the Jaro similarity score but this package defines it for any string distance.
	- [Partial](http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/) returns the minimum distance between the shorter string and substrings of the longer string.
	- [TokenSort](http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/) adjusts for differences in word orders by reording words alphabetically. 
	- [TokenSet](http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/) adjusts for differences in word orders and word numbers by comparing the intersection of two strings with each string.
		
## Usage

### The `str_distance::str_distance*` convenience functions.

`str_distance` and `str_distance_normalized` take the two string inputs for which the distance is determined using the passed 'DistanceMetric`.
`str_distance_normalized` evaluates the normalized distance between two strings. A value of '0.0' corresponds to the "zero distance", both strings are considered equal by means of the metric, whereas a value of '1.0' corresponds to the maximum distance that can exist between the strings.

Calling the `str_distance::str_distance*` is just convenience for `DistanceMetric.str_distance*("", "")` 

#### Example

Levenshtein metrics offer the possibility to define a maximum distance at which the further calculation of the exact distance is aborted early.

**Distance**

```rust
use str_distance::*;

// calculate the exact distance 
assert_eq!(str_distance("kitten", "sitting", Levenshtein::default()), DistanceValue::Exact(3));

// short circuit if distance exceeds 10
let s1 = "Wisdom is easily acquired when hiding under the bed with a saucepan on your head.";
let s2 = "The quick brown fox jumped over the angry dog.";
assert_eq!(str_distance(s1, s2, Levenshtein::with_max_distance(10)), DistanceValue::Exceeded(10));
```

**Normalized Distance**

```rust
use str_distance::*;
assert_eq!(str_distance_normalized("" , "", Levenshtein::default()), 0.0);
assert_eq!(str_distance_normalized("nacht", "nacht", Levenshtein::default()), 0.0);
assert_eq!(str_distance_normalized("abc", "def", Levenshtein::default()), 1.0);
```

### The `DistanceMetric` trait

```rust
use str_distance::{DistanceMetric, SorensenDice};
// QGram metrics require the length of the underlying fragment length to use for comparison.
// For `SorensenDice` default is 2.
assert_eq!(SorensenDice::new(2).str_distance("nacht", "night"), 0.75);

```

`DistanceMetric` was designed for `str` types, but is not limited to. Calculating distance is possible for all data types which are comparable and are passed as 'IntoIterator', e.g. as `Vec`

```rust
use str_distance::{DistanceMetric, Levenshtein, DistanceValue};

assert_eq!(*Levenshtein::default().distance(&[1,2,3], &[1,2,3,4,5,6]),3);
```


## Documentation

Full docs available at [docs.rs](https://docs.rs/str-distance)

## References

- [StringDistances](https://github.com/matthieugomez/StringDistances.jl)
- [The stringdist Package for Approximate String Matching](https://journal.r-project.org/archive/2014-1/loo.pdf) Mark P.J. van der Loo
- [fuzzywuzzy](http://chairnerd.seatgeek.com/fuzzywuzzy-fuzzy-string-matching-in-python/)


## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
   
