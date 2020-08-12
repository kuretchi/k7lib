//! Disjoint-set data structures (a.k.a. union-find data structures).

pub use self::quick_find::*;
pub use self::quick_union::*;

mod quick_find;
mod quick_union;
