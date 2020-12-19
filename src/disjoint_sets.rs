//! Disjoint-set data structures (a.k.a. union-find data structures).
//!
//! # References
//!
//! * [Galil, Z., & Italiano, G. F. (1991). Data structures and algorithms for disjoint set union problems. ACM Computing Surveys, 23(3), 319–344.][1]
//! * [Disjoint-set data structure - Wikipedia][2]
//!
//! [1]: https://doi.org/10.1145/116873.116878
//! [2]: https://en.wikipedia.org/w/index.php?title=Disjoint-set_data_structure&oldid=962428397

pub use self::quick_find::*;
pub use self::quick_union::*;

mod quick_find;
mod quick_union;
