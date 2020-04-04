use std::collections::HashMap;
use std::time::Instant;
use crate::combinations_with_split::CombinableWithSplit;
use crate::partition_combinations::PartitionCombinable;
use crate::mahjong_definitions::{Tiles, Deck, get_order, get_suite};


pub fn get_shanten(hand: &Vec<Tiles>)
{
  let now = Instant::now();
  let index_iterator: std::ops::Range<usize> = (0..12).into_iter();
  let presets: Vec<Vec<usize>> = index_iterator.partition_combinations(3).collect::<Vec<Vec<usize>>>();
  let pair_splits: Vec<[Vec<Tiles>; 2]> = hand.clone().into_iter().combinations_with_split(12).collect::<Vec<[Vec<Tiles>; 2]>>();
  println!("Presets {:?} + {:?}", presets.len(), pair_splits.len());
  println!("{}ms", now.elapsed().as_millis());
  let mut min_s = 14;
  let mut best_hand: Vec<Vec<Tiles>> = vec![];
  for [c, pair] in &pair_splits {
    for p in &presets {
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
  println!("{}ms (total)", now.elapsed().as_millis());
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

pub fn tiles_to_chance(tiles: &HashMap<Tiles, usize>, deck: &Deck) -> HashMap<Tiles, f32>
{
  let mut probability_map: HashMap<Tiles, f32> = HashMap::new();
  for (k, v) in tiles.iter()
  {
    let mut p: f32 = 1.0;
    for i in 0..*v
    {
      let p_i: f32 = (*deck.tiles.get(k).unwrap() as f32) / (deck.tile_count as f32);
      p *= p_i;
    }
    probability_map.insert(*k, p);
  }
  return probability_map;
}

pub fn complete_set(set: &Vec<Tiles>, i: usize) -> HashMap<Tiles, usize>
{
  let mut tile_map: HashMap<Tiles, usize> = HashMap::new();
  if triplet_analysis(set, i) == 2 || straight_analysis(set, i) == 2
  {
    return tile_map; // Set is 100% complete
  }
  // Triplet
  // Primary
  for j in 0..3
  {
    let tile: Tiles = set[j + i * 3];
    let need_count = 3 - set[0 + i * 3 .. 3 + i * 3].iter().filter(|n| **n == tile).count();
    tile_map.insert(tile, need_count);
    /*
      Cases
      A) Triplet is ready
      B) Triplet is missing 1 => 2 options 1 more of better, 2 more of worse
      C) All different => 3 options, 2 more of any
      D) Next turns you get new tile u start collecting
    */
  }
  return tile_map;
}


/*
  Tile utility: likelihood tile will be useful in end hand
   -> Tile that can crit in multiple sets is more useful
   For example 223388 will complete with 2,3,4,8, but 8 Is least useful
   If we consider subdivisions
   22X 33X 88X all tiles would seem equally useful having 2 a tile out. 
   Once we integrate over all possibilities
   23X 23X 88X etc. 23 should get a buff.

   Tile utility could be a sum of probabilities for all possible sets the tile is in possibille outs.

   Q: How to account for duplicated outs. In same example 23X 23X receiving a 4 will complete only 1 set, thus requiring another 4 to complete 2 sets.
   However in a easy integration implementation (set-wise) the algorithm would easily consider a single 4 to complete 2 sets.

   A: Hands should be calculated to completion. (eg. need 2x4, 1x8) and then the accurate probability can be counted.
   1) List Needed tiles from current set selection. If a set can be completed multiple ways (almost always) list options for sets and calculate all combinations. (at most (3^4 + some straights)*2 = likely some hunders of ways)
   2) For each combination of N tiles to completeness calculate probability to complete in M turns.
*/

