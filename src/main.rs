#![feature(test)]
#![feature(step_trait)]
#[macro_use]
mod impl_macros;
mod partition_combinations;
mod combinations_with_split;
mod mahjong_logic;
mod tests;
mod lazy_buffer;
extern crate test;

use mahjong_logic::{Tiles, get_shanten};

pub fn main() {
  // Please use 14 tiles for now =)
  // let hand: Vec<Tiles> = vec![Tiles::M1, Tiles::M1, Tiles::M1, Tiles::M4, Tiles::M6, Tiles::M9, Tiles::S4, Tiles::S1, Tiles::S4, Tiles::S9, Tiles::W, Tiles::Wh, Tiles::N, Tiles::S];
  let hand: Vec<Tiles> = vec![Tiles::P4, Tiles::P5, Tiles::P6, Tiles::M1, Tiles::M1, Tiles::M1, Tiles::S2, Tiles::S4, Tiles::S6, Tiles::S, Tiles::S, Tiles::Wh, Tiles::Wh, Tiles::G];
  get_shanten(hand)
}

