use itertools::Itertools;
use std::time::Instant;

#[derive(Copy, Clone)]
enum Tiles {
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
  let mut n;
  let mut set_combinations: Vec<Vec<usize>> = vec![];
  let twelve_tiles: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let twelve_vector: Vec<usize> = twelve_tiles.to_vec();
  for c in twelve_vector.into_iter().combinations(3) {
    if !c.contains(&0) {
      break;
    }
    n = 0;
    let mut nine_tiles: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    twelve_tiles.iter().for_each(|j| {
      if !c.contains(j) { nine_tiles[n] = *j; n += 1; }
    });
    let nine_first = nine_tiles[0];
    let nine_vector: Vec<usize> = nine_tiles.to_vec();
    for d in nine_vector.into_iter().combinations(3) {
      if !d.contains(&nine_first) {
        break;
      }
      n = 0;
      let mut six_tiles: [usize; 6] = [0, 0, 0, 0, 0, 0];
      nine_tiles.iter().for_each(|j| {
        if !d.contains(j) { six_tiles[n] = *j; n += 1; }
      });
      let six_first = six_tiles[0];
      let six_vector: Vec<usize> = six_tiles.to_vec();
      for e in six_vector.into_iter().combinations(3) {
        if !e.contains(&six_first) {
          break;
        }
        n = 0;
        let mut three_tiles: [usize; 3] = [0, 0, 0];
        six_tiles.iter().for_each(|j| {
          if !e.contains(j) { three_tiles[n] = *j; n += 1; }
        });
        let three_vector: Vec<usize> = three_tiles.to_vec();
        for f in three_vector.into_iter().combinations(3) {
          set_combinations.push(vec![c.clone(), d.clone(), e.clone(), f].into_iter().flatten().collect());
        }
      }
    }
  }
  set_combinations
}

pub fn yolo_main() {
  let now = Instant::now();
  let presets: Vec<Vec<usize>> = generate();
  let tiles: Vec<Tiles> = vec![Tiles::M1, Tiles::M1, Tiles::M2, Tiles::M4, Tiles::M6, Tiles::M9, Tiles::S4, Tiles::S1, Tiles::S4, Tiles::S9, Tiles::W, Tiles::Wh, Tiles::N, Tiles::S];
  for c in tiles.clone().into_iter().combinations(12) {
    for p in presets.clone() {
      // First set in index 0,1,2 second 3,4,5 etc.
      let values: Vec<Tiles> = p.iter().map(|i| c[*i].clone()).collect();
      let mut pair: Vec<Tiles>; // I dunno, just figure this out somehow. Maybe integrate into combinations for best performance
    }
  }
  println!("Amount of sets {:?}", presets.len());
  println!("{}", now.elapsed().as_millis());
  
}

