use super::_board::Board;
use super::coord::{Coord, Vector};
use super::r#move::Move;
use super::{Square::*, *};

pub struct DualCursor<'a> {
	target:         Coord,
	source:         Coord,
	target_changed: bool,
	board:          &'a mut Board,
	direction:      Vector,
	moves:          Vec<Move>
}

impl<'a> DualCursor<'a> {
	pub fn new(board: &'a mut Board, start: Coord, direction: Vector) -> Self {
		Self { target: start,
		       source: start.add(direction).unwrap(),
		       board,
		       direction,
		       target_changed: false,
		       moves: vec![] }
	}

	pub fn source_tile(&self) -> Square { self.board.at(self.source) }

	pub fn target_tile(&self) -> Square { self.board.at(self.target) }

	pub fn moves(self) -> Vec<Move> { self.moves }

	pub fn advance_both(&mut self) -> Result<(), ()> {
		self.advance_source()?; // order matters as advance_target() may also implicitly advance source if they are adjacent
		self.advance_target()
	}

	pub fn advance_source(&mut self) -> Result<(), ()> {
		match self.source.add(self.direction) {
			Ok(coord) => {
				self.source = coord;
				Ok(())
			},
			Err(()) => {
				// reached the end of the board => collect final Move::Stay if applicable
				self.push_unchanged_target();
				Err(())
			}
		}
	}

	pub fn advance_target(&mut self) -> Result<(), ()> {
		self.push_unchanged_target();
		self.target = self.target.add(self.direction).unwrap(); // target always smaller than source => cannot fail
		self.target_changed = false;
		if self.target == self.source {
			// source and target must not point to the same coord
			self.advance_source() // can fail
		} else {
			Ok(())
		}
	}

	pub fn move_tile(&mut self, tile_value: TileValue) -> () {
		self.board.put(self.target, Value(tile_value));
		self.board.put(self.source, Empty);
		self.target_changed = true;
		self.moves.push(Move::Shift { from: self.source, to: self.target, value: tile_value });
	}

	pub fn merge_tiles(&mut self, old_tile_value: TileValue, new_tile_value: TileValue) -> () {
		self.board.put(self.target, Value(new_tile_value));
		self.board.put(self.source, Empty);
		self.target_changed = true;
		self.moves.push(Move::Merge { from:        self.source,
		                              to:          self.target,
		                              start_value: old_tile_value,
		                              end_value:   new_tile_value });
	}

	fn push_unchanged_target(&mut self) -> () {
		if !self.target_changed {
			if let Value(target_value) = self.board.at(self.target) {
				self.moves.push(Move::Stay { at: self.target, value: target_value });
			}
		}
	}
}
