#![feature(test)]
#[macro_use]
mod impl_macros;
mod divisions;
mod simplified_divisions;
mod tests;
extern crate test;

use divisions::Combinable;
//use itertools::Itertools;
use std::time::Instant;

fn main() {
  let now = Instant::now();
  let a = (0..12).combinations(3);
  let it: Vec<Vec<u32>> = a.collect();
  //let it: Vec<Vec<u32>> = a.collect();
  println!("oijoi {:?}", it.len());
  println!("{}", now.elapsed().as_millis());
  println!("try simplified");
  simplified_divisions::yolo_main();
  /*let it2 = a.next();
  println!("oijoi2 {:?}", it2);
  let it3 = a.next();
  println!("oijoi3 {:?}", it3);*/
}
