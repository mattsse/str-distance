str-distance
=====================
[![Build Status](https://travis-ci.com/mattsse/str-distance.svg?branch=master)](https://travis-ci.com/mattsse/str-distance)
[![Crates.io](https://img.shields.io/crates/v/str-distance.svg)](https://crates.io/crates/str-distance)
[![Documentation](https://docs.rs/str-distance/badge.svg)](https://docs.rs/str-distance)

A crate to evaluate distances between str.

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
   
