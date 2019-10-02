use super::coord::{Coord, Vector};

pub struct DualCursor {
	pub target: Coord,
	pub source: Coord,
	direction: Vector,
}

impl DualCursor {
	pub fn new(start: Coord, direction: Vector) -> DualCursor {
		DualCursor { target: start, source: start.add(direction).unwrap(), direction }
	}

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
		if self.target == self.source {
			// source and target must not point to the same coord
			self.advance_source()?;
		}
		Ok(())
	}
}
