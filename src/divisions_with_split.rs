use std::fmt;
use super::lazy_buffer::LazyBuffer;

/// An iterator to iterate through all the `k`-length combinations in an iterator.
///
/// See [`.combinations()`](../trait.Itertools.html#method.combinations) for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct CombinationsWithSplit<I: Iterator> {
    indices: Vec<usize>,
    pool: LazyBuffer<I>,
    first: bool,
}

impl<I> Clone for CombinationsWithSplit<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(indices, pool, first);
}

impl<I> fmt::Debug for CombinationsWithSplit<I>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(Combinations, indices, pool, first);
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations_with_split<I>(iter: I, k: usize) -> CombinationsWithSplit<I>
    where I: Iterator
{
    let mut pool: LazyBuffer<I> = LazyBuffer::new(iter);

    for _ in 0..k {
        if !pool.get_next() {
            break;
        }
    }

    CombinationsWithSplit {
        indices: (0..k).collect(),
        pool,
        first: true,
    }
}

impl<I> Iterator for CombinationsWithSplit<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = [Vec<I::Item>; 2];
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if self.pool.is_done() {
                return None;
            }
            // Drain the pool, winter is coming
            while self.pool.get_next() {
            }
            self.first = false;
        } else if self.indices.len() == 0 {
            return None;
        } else {
            // Scan from the end, looking for an index to increment
            let mut i: usize = self.indices.len() - 1;

            // Check if we need to consume more from the iterator
            if self.indices[i] == self.pool.len() - 1 {
                self.pool.get_next(); // may change pool size
            }

            while self.indices[i] == i + self.pool.len() - self.indices.len() {
                if i > 0 {
                    i -= 1;
                } else {
                    // Reached the last combination
                    return None;
                }
            }

            // Increment index, and reset the ones to its right
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }

        // Create result vector based on the indices
        let mut counter_indices: Vec<usize> = (0..self.pool.len()).collect();
        let mut d = self.indices.clone();
        // self.indices are always ascending
        d.reverse();
        for j in d {
          counter_indices.remove(j);
        }
        Some([self.indices.iter().map(|i| self.pool[*i].clone()).collect(), counter_indices.iter().map(|i| self.pool[*i].clone()).collect()])
    }
}

pub trait CombinableWithSplit<I> 
  where I: Iterator
{
  fn combinations_with_split(self, k: usize) -> CombinationsWithSplit<I>;
}

impl CombinableWithSplit<std::ops::Range<u32>> for std::ops::Range<u32> {
  fn combinations_with_split(self, k: usize) -> CombinationsWithSplit<std::ops::Range<u32>> {
    combinations_with_split(self, k)
  }
}

impl<T> CombinableWithSplit<std::vec::IntoIter<T>> for std::vec::IntoIter<T> {
  fn combinations_with_split(self, k: usize) -> CombinationsWithSplit<std::vec::IntoIter<T>> {
    combinations_with_split(self, k)
  }
}

/*impl CombinableWithSplit<std::vec::IntoIter<simplified_divisions::Tiles>> for std::vec::IntoIter<simplified_divisions::Tiles> {
  fn combinations_with_split(self, k: usize) -> CombinationsWithSplit<std::vec::IntoIter<simplified_divisions::Tiles>> {
    combinations_with_split(self, k)
  }
}*/