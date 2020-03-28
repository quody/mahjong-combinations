use std::fmt;

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
  pool: Vec<I::Item>,
  stack: Stack,
  n: usize,
  first: bool,
  k: usize,
}

impl<I> Clone for Combinations<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(indices, pool, first, k, stack, n);
}

impl<I> fmt::Debug for Combinations<I>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(Combinations, indices, pool, first, k);
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations<I>(iter: I, k: usize) -> Combinations<I>
    where I: Iterator
{
    let pool: Vec<I::Item> = iter.into_iter().collect();
    let n = pool.len();

    Combinations {
        indices: (0..k).collect(),
        pool,
        n,
        first: true,
        k,
        stack: Stack::Empty
    }
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations_v(iter: Vec<usize>, k: usize) -> Combinations<std::vec::IntoIter<usize>>
{
    let pool: Vec<usize> = iter;
    let n = pool.len();

    Combinations {
        first: true,
        n,
        k,
        indices: (0..k).collect(),
        pool,
        stack: Stack::Empty
    }
}

impl<I> Iterator for Combinations<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = Vec<I::Item>; //, Vec<I::Item>);
    fn next(&mut self) -> Option<Self::Item> {
      if self.first {
        let l = self.pool.len();
        if l % self.k != 0 || l < self.k {
          return None;
        }

        // A reduced pool that has all values except the k values selected in this combination
        let remaining: Vec<usize> = (0..self.n).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
        if self.n - self.k > 0 {
          let new_node = Box::new(remaining.into_iter().combinations(self.k));
          self.stack = Stack::More(new_node);
        }
      }
      
      let mut i: usize = self.k - 1;
      let mut next_groups: Vec<usize> = vec![];
      
      match &mut self.stack {
        Stack::Empty => {
          if !self.first {
            return None
          }
        },
        Stack::More(it) => match it.next() {
          None => { 
            while self.indices[i] == i + self.n - self.k {
              // Call next
              if i > 1 {
                  i -= 1;
              } else {
                // Reached the last combination
                return None;
              }
            }

            self.indices[i] += 1;
            for j in i+1..self.k {
                self.indices[j] = self.indices[j - 1] + 1;
            }  

            // A reduced pool that has all values except the k values selected in this combination
            let remaining: Vec<usize> = (0..self.n).filter(|j| !self.indices.contains(j)).collect();
            let mut combo = remaining.into_iter().combinations(self.k);
            match combo.next() {
              None => (),
              Some(x) => next_groups = x,
            };
            let new_node = Box::new(combo);
            self.stack = Stack::More(new_node);

          },
          Some(x) => {
            next_groups = x;
          },
        }
      }

      let mut curr_values: Vec<I::Item> = self.indices.iter().map(|i| self.pool[*i].clone()).collect();
      // Map next_values (an array of arrays) indexes to their values
      let mut next_values: Vec<I::Item> = next_groups.iter().map(|i| self.pool[*i].clone()).collect();
      curr_values.append(&mut next_values);
      if self.first { 
        self.first = false; 
      }
      Some(curr_values)
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
