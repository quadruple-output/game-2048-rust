use super::cursor::DualCursor;
use super::r#move::Move;
use super::Square::*;

pub struct Merger<'a> {
	cursor: DualCursor<'a>
}

impl<'a> Merger<'a> {
	pub fn new(cursor: DualCursor<'a>) -> Self { Merger { cursor } }

	pub fn merge(mut self) -> Vec<Move> {
		self.merge_until_err().err(); // just ignore the result
		self.cursor.moves()
	}

	fn merge_until_err(&mut self) -> Result<(), ()> {
		loop {
			match self.cursor.source_tile() {
				Empty => {
					self.cursor.advance_source()?;
				},
				Value(source_value) => match self.cursor.target_tile() {
					Empty => {
						self.cursor.move_tile(source_value);
						self.cursor.advance_source()?;
					},
					Value(target_value) =>
						if target_value == source_value {
							self.cursor.merge_tiles(source_value, source_value + target_value);
							self.cursor.advance_both()?;
						} else {
							self.cursor.advance_target()?;
						},
				}
			}
		}
	}
}
