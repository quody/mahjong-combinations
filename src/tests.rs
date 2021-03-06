use crate::partition_combinations::PartitionCombinable;
use test::Bencher;

#[cfg(test)]
mod tests {
  use super::*;
  // Note this useful idiom: importing names from outer (for mod tests) scope.

  #[test]
  fn tiles_12() {
    let a = (0..12).partition_combinations(3);
    let it: usize = a.collect::<Vec<Vec<i32>>>().len();
    assert_eq!(it, 15400);
  }
  
  #[bench] 
  fn bench_tiles_12(b: &mut Bencher) 
  {
    b.iter(|| tiles_12())
  }
}