use super::_board::Board;
use super::coord::{Coord, Vector};
use super::{Square::*, *};

pub struct DualCursor<'a> {
	target:         Coord,
	source:         Coord,
	target_changed: bool,
	board:          &'a mut Board,
	direction:      Vector
}

impl<'a> DualCursor<'a> {
	pub fn new(board: &'a mut Board, start: Coord, direction: Vector) -> Self {
		Self { target: start, source: start.add(direction).unwrap(), board, direction, target_changed: false }
	}

	pub fn target_coord(&self) -> Coord { self.target }

	pub fn source_coord(&self) -> Coord { self.source }

	pub fn target_changed(&self) -> bool { self.target_changed }

	pub fn advance_both(&mut self) -> Result<(), ()> {
		self.advance_source()?; // order matters as advance_target() may also implicitly advance source if they are adjacent
		self.advance_target()?;
		Ok(())
	}

	pub fn advance_source(&mut self) -> Result<(), ()> {
		self.source = self.source.add(self.direction)?;
		Ok(())
	}

	pub fn advance_target(&mut self) -> Result<(), ()> {
		self.target = self.target.add(self.direction)?;
		self.target_changed = false;
		if self.target == self.source {
			// source and target must not point to the same coord
			self.advance_source()?;
		}
		Ok(())
	}

	pub fn source_tile(&self) -> Square { self.board.at(self.source) }

	pub fn target_tile(&self) -> Square { self.board.at(self.target) }

	pub fn move_tile_to_target(&mut self, tile_value: TileValue) -> () {
		self.board.put(self.target, Value(tile_value));
		self.board.put(self.source, Empty);
		self.target_changed = true;
	}
}
