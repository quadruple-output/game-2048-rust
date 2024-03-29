mod coord;
mod cursor;
mod merger;
mod r#move;
#[cfg(test)]
mod test;

pub use coord::Coord;
pub use r#move::Move;

use coord::Vector;
use cursor::DualCursor;
use merger::Merger;
use rand::{distributions::IndependentSample, Rng};
use std::cell::UnsafeCell;
use std::sync::mpsc;
use Square::*;

pub type TileValue = u16;

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views/console.rs, if possible
#[derive(PartialEq)] // needed for tests
pub enum Square {
  Empty,
  Value(TileValue),
}

type XYGrid = Vec<Vec<Square>>;

pub struct Randomizer {
  rand_range_10: rand::distributions::Range<u8>,
  rng: rand::ThreadRng,
}

impl Randomizer {
  pub fn new() -> Self {
    Randomizer { rand_range_10: rand::distributions::Range::new(0, 10), rng: rand::thread_rng() }
  }

  fn ten_percent_chance(&mut self) -> bool {
    self.rand_range_10.ind_sample(&mut self.rng) == 0
  }

  fn gen_int_less_than(&mut self, upper_bound: usize) -> usize {
    self.rng.gen_range(0, upper_bound)
  }
}

#[derive(Clone, Debug)]
pub struct Board {
  max_x: usize, // used as array index -> must be typed 'usize'
  max_y: usize, // used as array index -> must be typed 'usize'
  grid: XYGrid,
}

impl Board {
  pub fn new(size_x: usize, size_y: usize) -> Self {
    Board { max_x: size_x - 1, max_y: size_y - 1, grid: Self::empty_grid(size_x, size_y) }
  }

  pub fn coord(&self, x: usize, y: usize) -> Coord {
    Coord::new(x, y, self.max_x, self.max_y)
  }

  pub fn initialize(&mut self, randomizer: &mut Randomizer) -> Move {
    self.grid = Self::empty_grid(self.size_x(), self.size_y());
    self.new_tile(randomizer)
  }

  pub fn new_tile(&mut self, randomizer: &mut Randomizer) -> Move {
    let num_free_tiles = self.num_free_tiles();
    if num_free_tiles == 0 {
      panic!("tried to place a new tile on a full board")
    };
    let n = randomizer.gen_int_less_than(num_free_tiles);
    let rnd_free_coord = self.find_free_tile(n);
    let new_value = if randomizer.ten_percent_chance() { 4 } else { 2 };
    self.put(rnd_free_coord, Value(new_value));
    Move::Appear { at: rnd_free_coord, value: new_value }
  }

  pub fn size_x(&self) -> usize {
    self.max_x + 1
  }

  pub fn size_y(&self) -> usize {
    self.max_y + 1
  }

  pub fn at(&self, coord: Coord) -> Square {
    self.at_xy(coord.x, coord.y)
  }

  pub fn at_xy(&self, x: usize, y: usize) -> Square {
    self.grid[x][y]
  }

  pub fn put(&mut self, coord: Coord, square: Square) {
    self.grid[coord.x][coord.y] = square;
  }

  pub fn shift_left(&mut self) -> Option<Vec<Move>> {
    self.contract_multi(Vector::new(1, 0))
  }

  pub fn shift_right(&mut self) -> Option<Vec<Move>> {
    self.contract_multi(Vector::new(-1, 0))
  }

  pub fn shift_down(&mut self) -> Option<Vec<Move>> {
    self.contract_multi(Vector::new(0, -1))
  }

  pub fn shift_up(&mut self) -> Option<Vec<Move>> {
    self.contract_multi(Vector::new(0, 1))
  }

  fn empty_grid(size_x: usize, size_y: usize) -> XYGrid {
    vec![vec![Square::Empty; size_y]; size_x]
  }

  fn num_free_tiles(&self) -> usize {
    let mut n = 0;
    for column in &self.grid {
      for square in column {
        if let Empty = square {
          n += 1;
        }
      }
    }
    n
  }

  fn find_free_tile(&self, n: usize) -> Coord {
    let mut count = 0;
    for x in 0..self.size_x() {
      for y in 0..self.size_y() {
        if let Empty = self.grid[x][y] {
          if count == n {
            return self.coord(x, y);
          }
          count += 1;
        }
      }
    }
    panic!(); // n > self.num_free_tiles()
  }

  fn slice_in_direction(&mut self, direction: Vector) -> Vec<DualCursor> {
    let start_coords: Vec<Coord> = match direction {
      Vector { dx: 1, dy: 0 } => (0..=self.max_y).map(|y| self.coord(0, y)).collect(),
      Vector { dx: -1, dy: 0 } => (0..=self.max_y).map(|y| self.coord(self.max_x, y)).collect(),
      Vector { dx: 0, dy: 1 } => (0..=self.max_x).map(|x| self.coord(x, 0)).collect(),
      Vector { dx: 0, dy: -1 } => (0..=self.max_x).map(|x| self.coord(x, self.max_y)).collect(),
      _ => panic!(),
    };
    let unsafe_board: UnsafeCell<&mut Board> = UnsafeCell::new(self);
    unsafe {
      start_coords
        .into_iter()
        .map(|start_coord| DualCursor::new(*unsafe_board.get(), start_coord, direction))
        .collect()
    }
  }

  fn contract_multi(&mut self, direction: Vector) -> Option<Vec<Move>> {
    let (tx_clone_source, receiver) = mpsc::channel();
    crossbeam::scope(move |scope| {
      let cursors = self.slice_in_direction(direction);
      for cursor in cursors {
        let transmitter = tx_clone_source.clone();
        scope.spawn(move |_| {
          let my_moves = Merger::new(cursor).merge();
          for mv in my_moves {
            transmitter.send(mv).unwrap();
          }
        });
      }
    })
    .unwrap();
    let moves: Vec<Move> = receiver.into_iter().collect();
    // Return Some(moves) only if there are any _real_ moves. Otherwise return None:
    for mv in moves.iter() {
      match mv {
        Move::Stay { .. } => (),
        _ => return Some(moves),
      }
    }
    None
  }
}
