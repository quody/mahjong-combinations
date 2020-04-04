use enum_iterator::IntoEnumIterator;

pub struct Deck {
  pub tiles: std::collections::HashMap<Tiles, u32>,
  pub tile_count: u32
}

impl Deck {
  pub fn init(&mut self) {
    for tile in Tiles::into_enum_iter()
    {
      self.tiles.insert(tile, 4);
    }
    self.tile_count = 132;
  }

  pub fn remove_tiles(&mut self, tiles: &Vec<Tiles>)
  {
    for tile in tiles {
      self.tiles.insert(*tile, self.tiles.get(tile).unwrap() - 1);
      self.tile_count -= 1;
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suites {
  M,
  P,
  S,
  N // None
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, IntoEnumIterator)]
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

pub fn get_order(tile: Tiles) -> u32
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

pub fn get_suite(tile: Tiles) -> Suites
{
  return match tile {
    Tiles::M1 | Tiles::M2 | Tiles::M3 | Tiles::M4 | Tiles::M5 | Tiles::M6 | Tiles::M7 | Tiles::M8 | Tiles::M9 => Suites::M,
    Tiles::S1 | Tiles::S2 | Tiles::S3 | Tiles::S4 | Tiles::S5 | Tiles::S6 | Tiles::S7 | Tiles::S8 | Tiles::S9 => Suites::S,
    Tiles::P1 | Tiles::P2 | Tiles::P3 | Tiles::P4 | Tiles::P5 | Tiles::P6 | Tiles::P7 | Tiles::P8 | Tiles::P9 => Suites::P,
    _ => Suites::N
  }
}
