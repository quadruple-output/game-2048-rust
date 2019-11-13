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
	pub fn new(size_x: usize, size_y: usize) -> Self {
		Board { max_x:         size_x - 1,
		        max_y:         size_y - 1,
		        grid:          Self::empty_grid(size_x, size_y),
		        rand_range_10: rand::distributions::Range::new(0, 10),
		        rng:           rand::thread_rng() }
	}

	pub fn coord(&self, x: usize, y: usize) -> Coord { Coord::new(x, y, self.max_x, self.max_y) }

	pub fn initialize(&mut self) -> Move {
		self.grid = Self::empty_grid(self.size_x(), self.size_y());
		self.new_tile()
	}

	fn empty_grid(size_x: usize, size_y: usize) -> Vec<Vec<Square>> {
		vec![vec![Square::Empty; size_y]; size_x]
	}

	pub fn new_tile(&mut self) -> Move {
		let num_free_tiles = self.num_free_tiles();
		if num_free_tiles == 0 {
			panic!("tried to place a new tile on a full board")
		};
		let n = self.rng.gen_range(0, num_free_tiles);
		let rnd_free_coord = self.find_free_tile(n);
		let new_value = if self.ten_percent_chance() { 4 } else { 2 };
		self.put(rnd_free_coord, Value(new_value));
		Move::Appear { at: rnd_free_coord, value: new_value }
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

	pub fn shift_left(&mut self) -> Option<Vec<Move>> {
		self.contract_multi((0..=self.max_y).map(|y| self.coord(0, y)).collect(), Vector::new(1, 0))
	}

	pub fn shift_right(&mut self) -> Option<Vec<Move>> {
		self.contract_multi((0..=self.max_y).map(|y| self.coord(self.max_x, y)).collect(), Vector::new(-1, 0))
	}

	pub fn shift_down(&mut self) -> Option<Vec<Move>> {
		self.contract_multi((0..=self.max_x).map(|x| self.coord(x, self.max_y)).collect(), Vector::new(0, -1))
	}

	pub fn shift_up(&mut self) -> Option<Vec<Move>> {
		self.contract_multi((0..=self.max_x).map(|x| self.coord(x, 0)).collect(), Vector::new(0, 1))
	}

	fn contract_multi(&mut self, starts: Vec<Coord>, direction: Vector) -> Option<Vec<Move>> {
		let mut moves = Vec::with_capacity(self.size_x() * self.size_y());
		for start_coord in starts {
			moves.append(&mut self.contract(DualCursor::new(start_coord, direction)));
		}
		// Return moves only if there are any _real_ moves:
		// TODO: reduce to single statement (filter_map?)
		for mv in moves.iter() {
			match mv {
				Move::Stay { .. } => (),
				_ => return Some(moves)
			}
		}
		None
	}

	fn contract(&mut self, mut cursor: DualCursor) -> Vec<Move> {
		let mut moves = Vec::new();
		let mut target_changed = false;
		loop {
			match self.at(cursor.source) {
				Empty => {
					if cursor.advance_source().is_err() {
						if !target_changed {
							if let Value(target_value) = self.at(cursor.target) {
								moves.push(Move::Stay { at: cursor.target, value: target_value });
							}
						}
						break;
					};
				},
				Value(source_value) => match self.at(cursor.target) {
					Empty => {
						moves.push(Move::Shift { from: cursor.source, to: cursor.target, value: source_value });
						self.put(cursor.target, Value(source_value));
						self.put(cursor.source, Empty);
						target_changed = true;
						if cursor.advance_source().is_err() {
							break;
						}
					},
					Value(target_value) =>
						if target_value == source_value {
							if !target_changed {
								moves.push(Move::Stay { at: cursor.target, value: target_value });
							}
							moves.push(Move::Merge { from:        cursor.source,
							                         to:          cursor.target,
							                         start_value: source_value,
							                         end_value:   source_value + target_value });
							self.put(cursor.target, Value(source_value + target_value));
							self.put(cursor.source, Empty);
							if cursor.advance_both().is_err() {
								break;
							};
							target_changed = false;
						} else {
							if !target_changed {
								moves.push(Move::Stay { at: cursor.target, value: target_value });
							}
							if cursor.advance_target().is_err() {
								// reached end of board while target has a value
								moves.push(Move::Stay { at: cursor.source, value: source_value });
								break;
							};
							target_changed = false;
						},
				}
			}
		}
		moves
	}
}
