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
    stack_size: usize,
}

impl<I> Clone for Combinations<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(indices, next_groups, pool, first, k, stack, stack_size);
}

impl<I> fmt::Debug for Combinations<I>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(Combinations, next_groups, indices, pool, first, k, stack_size);
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
        stack: Stack::Empty,
        stack_size: 0,
    }
}

impl<I> Iterator for Combinations<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = Vec<Vec<I::Item>>; //, Vec<I::Item>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if self.pool.is_done() {
                return None;
            }
            while self.pool.get_next() {
            }
            let remaining: Vec<usize> = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
            self.stack_size = remaining.len();
            if remaining.len() >= self.k {
              let new_node = Box::new(remaining.into_iter().combinations(self.k));
              self.stack = Stack::More(new_node);
              println!("init first stack amiright");
            }
            self.first = false;
            println!("first amiright {:?}", self.pool.len());
            match self.stack.clone() {
              Stack::Empty => {
                println!("1taivas varjele");
              },
              Stack::More(mut it) => match (*it).next() {
                None => { 
                  println!("1taivas varjele2");
                  self.stack = Stack::Empty;
                },
                Some(x) => {
                  println!("1taivas varjele3 {:?}", x);
                  self.next_groups = x;
                  self.stack = Stack::More(it);
                },
              }
            }
        } else if self.indices.len() == 0 {
            return None;
        } else if let Stack::Empty = self.stack {
            // Scan from the end, looking for an index to increment
            let mut i: usize = self.indices.len() - 1;

            // Check if we need to consume more from the iterator
            if self.indices[i] == self.pool.len() - 1 {
                self.pool.get_next(); // may change pool size
            }

            while self.indices[i] == i + self.pool.len() - self.indices.len() {
                // Call next
                if i > 0 {
                    i -= 1;
                } else {
                    // Reached the last combination
                    return None;
                }
            }

            // Reached the last relevant combination
            if i == 0 {
              return None;
            }

            // Increment index, and reset the ones to its right
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
            // Reset stack
            let remaining: Vec<usize> = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
            if remaining.len() >= self.k {
              println!("reset stack =)");
              let new_node = Box::new(remaining.into_iter().combinations(3));
              self.stack = Stack::More(new_node);
            }
        } else {
          println!("something");
          match self.stack.clone() {
            Stack::Empty => {
              println!("taivas varjele111");
            },
            Stack::More(mut it) => match (*it).next() {
              None => { 
                println!("taivas varjele2");
                self.stack = Stack::Empty;
                // Go earlOer
              },
              Some(x) => {
                println!("taivas varjele3 {:?}", x);
                self.next_groups = x
              },
            },
          };
        }

        // Code quarantine

        if let Stack::Empty = self.stack 
        {
            if self.stack_size > 0 {
            // Scan from the end, looking for an index to increment
            let mut i: usize = self.indices.len() - 1;

            // Check if we need to consume more from the iterator
            if self.indices[i] == self.pool.len() - 1 {
                self.pool.get_next(); // may change pool size
            }

            while self.indices[i] == i + self.pool.len() - self.indices.len() {
                // Call next
                if i > 0 {
                    i -= 1;
                } else {
                    // Reached the last combination
                    return None;
                }
            }

            // Reached the last relevant combination
            if i == 0 {
              return None;
            }

            // Increment index, and reset the ones to its right
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
            // Reset stack
            /*let remaining: Vec<usize> = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
            if remaining.len() >= self.k {
              println!("reset stack =)");
              let new_node = Box::new(remaining.into_iter().combinations(3));
              self.stack = Stack::More(new_node);
            }*/

            let remaining: Vec<usize> = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
            self.stack_size = remaining.len();
            if remaining.len() >= self.k {
              let new_node = Box::new(remaining.into_iter().combinations(self.k));
              self.stack = Stack::More(new_node);
              println!("init first stack amiright");
            }
            println!("first amiright {:?}", self.pool.len());
            match self.stack.clone() {
              Stack::Empty => {
                println!("1taivas varjele");
              },
              Stack::More(mut it) => match (*it).next() {
                None => { 
                  println!("1taivas varjele2");
                  self.stack = Stack::Empty;
                },
                Some(x) => {
                  println!("1taivas varjele3 {:?}", x);
                  self.next_groups = x;
                  self.stack = Stack::More(it);
                },
              }
            }
          } 
        }

        // CODE QUARANTINE ENDS

        // Create result vector based on the indices
        //let remaining = (0..self.pool.len()).filter(|j| !self.indices.contains(j)).collect::<Vec<usize>>();
        let curr_values: Vec<I::Item> = self.indices.iter().map(|i| self.pool[*i].clone()).collect();
        let curr_clone = curr_values.clone();
        let next_values = self.next_groups.iter().map(
          |set| set.iter().map(
            |i| self.pool[*i].clone()
          ).collect()).collect();
          let all_values = vec![vec![curr_values], next_values].concat();
          println!("returning {:?} {:?}", self.indices, self.next_groups);
          if self.next_groups.len() == 0 {
            self.stack = Stack::Empty;
            //return None;
            return Some(vec![curr_clone]);
          }
        Some(all_values)
        //Some(self.indices.iter().map(|i| self.pool[*i].clone()).collect()) //, remaining.iter().map(|i| self.pool[*i].clone()).collect()))
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