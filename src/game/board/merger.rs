use super::cursor::DualCursor;
use super::r#move::Move;
use super::Square::*;

pub struct Merger<'a> {
	cursor: DualCursor<'a>
}

impl<'a> Merger<'a> {
	pub fn new(cursor: DualCursor<'a>) -> Self { Merger { cursor } }

	pub fn merge(self) -> Vec<Move> {
		let mut moves = Vec::new();
		self.merge_until_err(&mut moves).err(); // just ignore the result
		moves
	}

	fn merge_until_err(mut self, moves: &mut Vec<Move>) -> Result<(), ()> {
		loop {
			match self.cursor.source_tile() {
				Empty => {
					self.cursor.advance_source(moves)?;
				},
				Value(source_value) => match self.cursor.target_tile() {
					Empty => {
						self.cursor.move_tile(source_value, moves);
						self.cursor.advance_source(moves)?;
					},
					Value(target_value) =>
						if target_value == source_value {
							self.cursor.merge_tiles(source_value, source_value + target_value, moves);
							self.cursor.advance_both(moves)?;
						} else {
							self.cursor.advance_target(moves)?;
						},
				}
			}
		}
	}
}
