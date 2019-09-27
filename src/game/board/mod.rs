mod board;
mod coord;
mod cursor;
#[cfg(test)]
mod test;

pub use board::Board;

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views/console.rs, if possible
#[derive(PartialEq)] // needed for tests
pub enum Square {
    Empty,
    Value(u16),
}