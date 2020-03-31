use std::time::Instant;
use crate::combinations_with_split::CombinableWithSplit;
use crate::partition_combinations::PartitionCombinable;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suites {
  M,
  P,
  S,
  N // None
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tiles {
  M1,
  M2,
  M3,
  M4,
  M5,
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

fn get_order(tile: Tiles) -> u32
{
  return match tile {
    Tiles::M1 | Tiles::S1 | Tiles::P1 => 1,
    Tiles::M2 | Tiles::S2 | Tiles::P2 => 2,
    Tiles::M3 | Tiles::S3 | Tiles::P3 => 3,
    Tiles::M4 | Tiles::S4 | Tiles::P4 => 4,
    Tiles::M5 | Tiles::S5 | Tiles::P5 => 5,
    Tiles::M6 | Tiles::S6 | Tiles::P6 => 6,
    Tiles::M7 | Tiles::S7 | Tiles::P7 => 7,
    Tiles::M8 | Tiles::S8 | Tiles::P8 => 8,
    Tiles::M9 | Tiles::S9 | Tiles::P9 => 9,
    _ => 1000
  }
}

fn get_suite(tile: Tiles) -> Suites
{
  return match tile {
    Tiles::M1 | Tiles::M2 | Tiles::M3 | Tiles::M4 | Tiles::M5 | Tiles::M6 | Tiles::M7 | Tiles::M8 | Tiles::M9 => Suites::M,
    Tiles::S1 | Tiles::S2 | Tiles::S3 | Tiles::S4 | Tiles::S5 | Tiles::S6 | Tiles::S7 | Tiles::S8 | Tiles::S9 => Suites::S,
    Tiles::P1 | Tiles::P2 | Tiles::P3 | Tiles::P4 | Tiles::P5 | Tiles::P6 | Tiles::P7 | Tiles::P8 | Tiles::P9 => Suites::P,
    _ => Suites::N
  }
}

pub fn get_shanten(hand: Vec<Tiles>)
{
  let now = Instant::now();
  let index_iterator: std::ops::Range<usize> = (0..12).into_iter();
  let presets: Vec<Vec<usize>> = index_iterator.partition_combinations(3).collect::<Vec<Vec<usize>>>();
  println!("Presets {:?}", presets.len());
  println!("{}ms", now.elapsed().as_millis());
  let mut min_s = 14;
  let mut best_hand: Vec<Vec<Tiles>> = vec![];
  for [c, pair] in hand.clone().into_iter().combinations_with_split(12) {
    for p in presets.clone() {
      // First set in index 0,1,2 second 3,4,5 etc.
      let sets: Vec<Tiles> = p.iter().map(|i| c[*i].clone()).collect();
      let mut s = shanten_helper(&sets);
      if pair[0] == pair[1] {
        s -= 1;
      }
      if s < min_s {
        min_s = s;
        best_hand = vec![sets.clone(), pair.clone()];
      }
      
    }
  }
  println!("Analysis ok.");
  println!("Minimum {}-shanten {:?}", min_s, best_hand);
  println!("{}ms", now.elapsed().as_millis());
}

fn shanten_helper(set: &Vec<Tiles>) -> u32 {
  let mut shanten = 8;
  for i in 0..4 {
    shanten -= std::cmp::max(triplet_analysis(set, i), straight_analysis(set, i));
  }
  return shanten;
}




pub fn triplet_analysis(set: &Vec<Tiles>, i: usize) -> u32
{
  let mut shanten_reduction = 0;
  if set[i * 3 + 0] == set[i * 3 + 1] { 
    shanten_reduction += 1; 
  }
  if set[i * 3 + 1] == set[i * 3 + 2] {
    shanten_reduction += 1;
  }
  return shanten_reduction;
}

pub fn straight_analysis(set: &Vec<Tiles>, i: usize) -> u32
{
  let order_0 = get_order(set[i * 3 + 0]);
  let order_1 = get_order(set[i * 3 + 1]);
  let order_2 = get_order(set[i * 3 + 2]);
  let suite_0 = get_suite(set[i * 3 + 0]);
  let suite_1 = get_suite(set[i * 3 + 1]);
  let suite_2 = get_suite(set[i * 3 + 2]);
  // Maybe check suite here as well!
  if (order_0 + 1 == order_1) && suite_0 == suite_1 && (order_1 + 1 == order_2) && suite_1 == suite_2
  {
    return 2;
  }
  // If original hand is sorted, we can assume ordered triplet.
  else if (order_0 + 1 == order_1 && suite_0 == suite_1) || (order_1 + 1 == order_2 && suite_1 == suite_2)
  {
    return 1;
  }
  else if (order_0 + 2 == order_1 && suite_0 == suite_2) || (order_1 + 2 == order_2 && suite_1 == suite_2)
  {
    return 1;
  }
  return 0;
}

