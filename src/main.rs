#![feature(test)]
#![feature(step_trait)]
#[macro_use]
mod impl_macros;
mod partition_combinations;
mod combinations_with_split;
mod mahjong_logic;
mod mahjong_definitions;
mod tests;
mod lazy_buffer;
extern crate test;
use std::collections::HashMap;

use mahjong_logic::{get_shanten, complete_set, tiles_to_chance};
use mahjong_definitions::{Tiles, Deck};

pub fn main() {
  // Please use 14 tiles for now =) Tiles should be ordered
  // let hand: Vec<Tiles> = vec![Tiles::M1, Tiles::M1, Tiles::M1, Tiles::M4, Tiles::M6, Tiles::M9, Tiles::S4, Tiles::S1, Tiles::S4, Tiles::S9, Tiles::W, Tiles::Wh, Tiles::N, Tiles::S];
  let hand: Vec<Tiles> = vec![Tiles::P4, Tiles::P5, Tiles::P6, Tiles::M1, Tiles::M1, Tiles::M1, Tiles::S2, Tiles::S2, Tiles::S6, Tiles::S, Tiles::S, Tiles::Wh, Tiles::Wh, Tiles::G];
  // get_shanten(&hand);
  let mut deck: Deck = Deck { tiles: HashMap::new(), tile_count: 0 };
  deck.init();
  deck.remove_tiles(&hand);
  let moi = tiles_to_chance(&complete_set(&hand, 2), &deck);
  println!("{:?}", moi);
}

