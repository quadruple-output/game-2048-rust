use super::coord::{Coord, Vector};
use super::cursor::DualCursor;
use super::r#move::Move;
use super::{Square, Square::*};
use rand::{distributions::IndependentSample, Rng};

#[derive(Clone, Debug)]
pub struct Board {
	max_x:         usize, // used as array index -> must be typed 'usize'
	max_y:         usize, // used as array index -> must be typed 'usize'
	grid:          Vec<Vec<Square>>,
	rand_range_10: rand::distributions::Range<u8>,
	rng:           rand::ThreadRng
}

impl PartialEq for Board {
	fn eq(&self, other: &Board) -> bool {
		if self.max_x != other.max_x || self.max_y != other.max_y {
			false
		} else {
			for x in 0..self.size_x() {
				for y in 0..self.size_y() {
					if self.grid[x][y] != other.grid[x][y] {
						return false;
					}
				}
			}
			true
		}
	}
}

impl Board {
	pub fn new() -> Board {
		let (size_x, size_y) = (4, 4);
		Board { max_x:         size_x - 1,
		        max_y:         size_y - 1,
		        grid:          vec![vec![Square::Empty; size_y]; size_x],
		        rand_range_10: rand::distributions::Range::new(0, 10),
		        rng:           rand::thread_rng() }
	}

	pub fn coord(&self, x: usize, y: usize) -> Coord { Coord::new(x, y, self.max_x, self.max_y) }

	pub fn initialize(&mut self) {
		self.grid = vec![vec![Square::Empty; self.size_y()]; self.size_x()];
		self.new_tile();
	}

	pub fn new_tile(&mut self) {
		let num_free_tiles = self.num_free_tiles();
		if num_free_tiles == 0 {
			panic!("tried to place a new tile on a full board")
		};
		let n = self.rng.gen_range(0, num_free_tiles);
		let rnd_free_coord = self.find_free_tile(n);
		let new_value = if self.ten_percent_chance() { 4 } else { 2 };
		self.put(rnd_free_coord, Value(new_value));
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
		panic!();
	}

	pub fn size_x(&self) -> usize { self.max_x + 1 }

	pub fn size_y(&self) -> usize { self.max_y + 1 }

	pub fn at(&self, coord: Coord) -> Square { self.at_xy(coord.x, coord.y) }

	pub fn at_xy(&self, x: usize, y: usize) -> Square { self.grid[x][y] }

	pub fn put(&mut self, coord: Coord, square: Square) { self.grid[coord.x][coord.y] = square; }

	fn ten_percent_chance(&mut self) -> bool { self.rand_range_10.ind_sample(&mut self.rng) == 0 }

	pub fn shift_left(&mut self) -> Result<Vec<Move>, ()> {
		self.contract_multi((0..=self.max_y).map(|y| self.coord(0, y)).collect(), Vector::new(1, 0))
	}

	pub fn shift_right(&mut self) -> Result<Vec<Move>, ()> {
		self.contract_multi((0..=self.max_y).map(|y| self.coord(self.max_x, y)).collect(), Vector::new(-1, 0))
	}

	pub fn shift_down(&mut self) -> Result<Vec<Move>, ()> {
		self.contract_multi((0..=self.max_x).map(|x| self.coord(x, self.max_y)).collect(), Vector::new(0, -1))
	}

	pub fn shift_up(&mut self) -> Result<Vec<Move>, ()> {
		self.contract_multi((0..=self.max_x).map(|x| self.coord(x, 0)).collect(), Vector::new(0, 1))
	}

	fn contract_multi(&mut self, starts: Vec<Coord>, direction: Vector) -> Result<Vec<Move>, ()> {
		let mut moves = Vec::with_capacity(self.size_x() * self.size_y());
		for start_coord in starts {
			moves.append(&mut self.contract(DualCursor::new(start_coord, direction)));
		}
		if moves.is_empty() { Err(()) } else { Ok(moves) }
	}

	fn contract(&mut self, mut cursor: DualCursor) -> Vec<Move> {
		let mut result = Vec::new();
		loop {
			match self.at(cursor.source) {
				Empty => {
					if let Err(_) = cursor.advance_source() {
						break;
					};
				},
				Value(source_value) => match self.at(cursor.target) {
					Empty => {
						result.push(Move::new(cursor.source, cursor.target, source_value, source_value));
						self.put(cursor.target, Value(source_value));
						self.put(cursor.source, Empty);
						if let Err(_) = cursor.advance_source() {
							break;
						}
					},
					Value(target_value) =>
						if target_value == source_value {
							result.push(Move::new(cursor.source, cursor.target, source_value, source_value + target_value));
							self.put(cursor.target, Value(source_value + target_value));
							self.put(cursor.source, Empty);
							if let Err(_) = cursor.advance_both() {
								break;
							};
						} else {
							if let Err(_) = cursor.advance_target() {
								break;
							};
						},
				}
			}
		}
		result
	}
}
