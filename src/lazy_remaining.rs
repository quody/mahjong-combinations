use std::ops::Index;
use super::lazy_buffer::LazyBuffer;

#[derive(Debug, Clone)]
pub struct LazyRemaining<I: Iterator> {
    pub it: I,
    pub remaining: I,
    next,
    size_hint,
    count,
    last,
    nth,
    chain
    count,
  }
  
  impl Iterator for LazyRemaining {
    fn next(&mut self) -> Option<Self::Item> {

    }
    fn size_hint(&self) -> (usize, Option<usize>),
    fn count(self) -> usize
    fn last(self) -> Option<Self::Item>
    fn nth(&mut self, mut n: usize) -> Option<Self::Item>,
    fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
}