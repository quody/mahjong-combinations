use std::fmt;
use super::lazy_buffer::LazyBuffer;

/// An iterator to iterate through all the `k`-length combinations in an iterator.
///
/// See [`.combinations()`](../trait.Itertools.html#method.combinations) for more information.

pub enum Stack {
  Empty,
  More(Box<Combinations<std::vec::IntoIter<usize>>>),
}

impl Clone for Stack
{
    fn clone(&self) -> Stack {
      match self {
        Stack::Empty => Stack::Empty,
        Stack::More(x) => Stack::More(x.clone())
      }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Combinations<I: Iterator> {
    indices: Vec<usize>,
    next_groups: Vec<Vec<usize>>,
    pool: LazyBuffer<I>,
    first: bool,
    k: usize,
    stack: Stack,
}

impl<I> Clone for Combinations<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(indices, next_groups, pool, first, k, stack);
}

impl<I> fmt::Debug for Combinations<I>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(Combinations, next_groups, indices, pool, first, k);
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations<I>(iter: I, k: usize) -> Combinations<I>
    where I: Iterator
{
    let mut pool: LazyBuffer<I> = LazyBuffer::new(iter);

    for _ in 0..k {
        if !pool.get_next() {
            break;
        }
    }

    Combinations {
        indices: (0..k).collect(),
        next_groups: vec![],
        pool,
        first: true,
        k,
        stack: Stack::Empty
    }
}

impl<I> Iterator for Combinations<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = Vec<Vec<I::Item>>; //, Vec<I::Item>);
    fn next(&mut self) -> Option<Self::Item> {
      if self.first {
        while self.pool.get_next() {
        }

        let l = self.pool.len();
        if l % self.k != 0 || l < self.k {
          return None;
        }

        let remaining: Vec<usize> = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
        if remaining.len() > 0 {
          let new_node = Box::new(remaining.into_iter().combinations(self.k));
          self.stack = Stack::More(new_node);
        }
      }
      
      let mut i: usize = self.indices.len() - 1;
      
      match &mut self.stack {
        Stack::Empty => {
          if self.first {
            self.next_groups = vec![]
          } else {
            while self.indices[i] == i + self.pool.len() - self.indices.len() {
              // Call next
              if i > 0 {
                  i -= 1;
              } else {
                return None;
              }
            }
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
          }
        },
        Stack::More(it) => match it.next() {
          None => { 
            while self.indices[i] == i + self.pool.len() - self.indices.len() {
              // Call next
              if i > 0 {
                  i -= 1;
              } else {
                // Reached the last combination
                return None;
              }
            }
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }

            let remaining: Vec<usize> = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
            if remaining.len() > 0 {
              let mut combo = remaining.into_iter().combinations(self.k);
              self.next_groups = match combo.next() {
                None => vec![],
                Some(x) => x
              };
              let new_node = Box::new(combo);
              self.stack = Stack::More(new_node);
            }

          },
          Some(x) => {
            self.next_groups = x;
            // self.stack = Stack::More(it);
          },
        }
      }

      if i == 0 {
        return None;
      }

      let curr_values: Vec<I::Item> = self.indices.iter().map(|i| self.pool[*i].clone()).collect();
      let next_values = self.next_groups.iter().map(
        |set| set.iter().map(
          |i| self.pool[*i].clone()
        ).collect()).collect();
        let all_values = vec![vec![curr_values], next_values].concat();
      self.first = false;
      Some(all_values)
    }
}

pub trait Combinable<I> 
  where I: Iterator
{
  fn combinations(self, k: usize) -> Combinations<I>;
}

impl Combinable<std::ops::Range<u32>> for std::ops::Range<u32> {
  fn combinations(self, k: usize) -> Combinations<std::ops::Range<u32>> {
    combinations(self, k)
  }
}

impl Combinable<std::vec::IntoIter<usize>> for std::vec::IntoIter<usize> {
  fn combinations(self, k: usize) -> Combinations<std::vec::IntoIter<usize>> {
    combinations(self, k)
  }
}