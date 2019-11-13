mod _board;
mod coord;
mod cursor;
mod r#move;
#[cfg(test)]
mod test;

pub use _board::Board;
pub use coord::Coord;
pub use r#move::Move;

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views/console.rs, if possible
#[derive(PartialEq)] // needed for tests
pub enum Square {
	Empty,
	Value(u16)
}
