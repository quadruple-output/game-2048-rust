mod board;
mod coord;
mod cursor;
mod merger;
mod r#move;
#[cfg(test)]
mod test;

pub use board::Board;
pub use coord::Coord;
pub use r#move::Move;

pub type TileValue = u16;

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views/console.rs, if possible
#[derive(PartialEq)] // needed for tests
pub enum Square {
  Empty,
  Value(TileValue)
}
