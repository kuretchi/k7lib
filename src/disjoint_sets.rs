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

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  #[derive(Clone, PartialEq, Eq, Debug)]
  pub struct NaiveDisjointSets(pub Vec<Option<HashSet<usize>>>);

  impl NaiveDisjointSets {
    pub fn sets_len(&self) -> usize {
      self.0.iter().filter(|&set| set.is_some()).count()
    }

    pub fn find(&self, i: usize) -> usize {
      self
        .0
        .iter()
        .position(|set| set.as_ref().map_or(false, |set| set.contains(&i)))
        .unwrap()
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
      let i = self.find(i);
      let j = self.find(j);
      if i == j {
        return false;
      }
      let s = self.0[j].take().unwrap();
      self.0[i].as_mut().unwrap().extend(s);
      true
    }

    pub fn set(&self, i: usize) -> &HashSet<usize> {
      self.0[self.find(i)].as_ref().unwrap()
    }

    pub fn set_len(&self, i: usize) -> usize {
      self.set(i).len()
    }
  }
}
