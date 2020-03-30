use crate::divisions_with_split::CombinableWithSplit;
use itertools::Itertools;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
pub enum Tiles {
  M1,
  M2,
  M3,
  M4,
  M6,
  M7,
  M8,
  M9,
  P1,
  P2,
  P3,
  P4,
  P5,
  P6,
  P7,
  P8,
  P9,
  S1,
  S2,
  S3,
  S4,
  S5,
  S6,
  S7,
  S8,
  S9,
  E, //East
  S, //South
  W, //West
  N, //North
  Wh, //White
  G, //Green
  R //Red
}

fn filter_ones() {
  let arr: Vec<u32> = (0..1000000).collect();
  let now = Instant::now();
  let sets: Vec<u32> = arr.into_iter().filter(|n| !n.to_string().contains("1")).collect();
  println!("Amount of sets {:?}", sets.len());
  println!("{}", now.elapsed().as_millis());
}

fn generate() -> Vec<Vec<usize>> {
  // Save list of index combinations, use them to collect from 14 choose 12 tiles? Faster than 91x this?
  let mut set_combinations: Vec<Vec<usize>> = vec![];
  let twelve_left: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  for c in twelve_left.clone().into_iter().combinations(3) {
    if !c.contains(&0) {
      break;
    }
    let nine_left = twelve_left.clone().into_iter().filter(|j| !c.contains(j)).collect::<Vec<usize>>();
    let nine_first = nine_left[0];
    for d in nine_left.clone().into_iter().combinations(3) {
      if !d.contains(&nine_first) {
        break;
      }
      let six_left = nine_left.clone().into_iter().filter(|j| !d.contains(j)).collect::<Vec<usize>>();
      let six_first = six_left[0];
      for e in six_left.clone().into_iter().combinations(3) {
        if !e.contains(&six_first) {
          break;
        }
        let three_left = six_left.clone().into_iter().filter(|j| !e.contains(j)).collect::<Vec<usize>>();
        set_combinations.push(vec![c.clone(), d.clone(), e.clone(), three_left].into_iter().flatten().collect());
      }
    }
  }
  set_combinations
}

pub fn yolo_main() {
  let now = Instant::now();
  let presets: Vec<Vec<usize>> = generate();
  let hand: Vec<Tiles> = vec![Tiles::M1, Tiles::M1, Tiles::M2, Tiles::M4, Tiles::M6, Tiles::M9, Tiles::S4, Tiles::S1, Tiles::S4, Tiles::S9, Tiles::W, Tiles::Wh, Tiles::N, Tiles::S];
  for [c, pair] in hand.clone().into_iter().combinations_with_split(12) {
    for p in presets.clone() {
      let shanten = 0;
      // First set in index 0,1,2 second 3,4,5 etc.
      let sets: Vec<Tiles> = p.iter().map(|i| c[*i].clone()).collect();
      if pair[0] != pair[1] {
        // println!("Not even a pair! GG!");
      }
    }
  }
  println!("Amount of sets per selected pair {:?}", presets.len());
  println!("{}ms", now.elapsed().as_millis());
}

