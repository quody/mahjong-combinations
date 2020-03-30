use itertools::Itertools;
use std::time::Instant;

fn generate() -> Vec<Vec<Vec<usize>>> {
  let mut n;
  let mut set_combinations: Vec<Vec<Vec<usize>>> = vec![];
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
          set_combinations.push(vec![c.clone(), d.clone(), e.clone(), f]);
        }
      }
    }
  }
  set_combinations
}

pub fn yolo_main() {
  let now = Instant::now();
  let sets = generate();
  println!("Amount of sets {:?}", sets.len());
  println!("{}", now.elapsed().as_millis());
}

