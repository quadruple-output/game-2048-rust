use super::cursor::DualCursor;
use super::r#move::Move;
use super::Square::*;

pub struct Merger<'a> {
	cursor: DualCursor<'a>
}

impl<'a> Merger<'a> {
	pub fn new(cursor: DualCursor<'a>) -> Self { Merger { cursor } }

	pub fn merge(mut self) -> Vec<Move> {
		let mut moves = Vec::new();
		loop {
			match self.cursor.source_tile() {
				Empty => {
					if self.cursor.advance_source().is_err() {
						if !self.cursor.target_changed {
							if let Value(target_value) = self.cursor.target_tile() {
								moves.push(Move::Stay { at: self.cursor.target, value: target_value });
							}
						}
						break;
					};
				},
				Value(source_value) => match self.cursor.target_tile() {
					Empty => {
						self.cursor.move_to_target(source_value);
						moves.push(Move::Shift { from:  self.cursor.source,
						                         to:    self.cursor.target,
						                         value: source_value });
						if self.cursor.advance_source().is_err() {
							break;
						}
					},
					Value(target_value) =>
						if target_value == source_value {
							if !self.cursor.target_changed {
								moves.push(Move::Stay { at: self.cursor.target, value: target_value });
							}
							let merged_value = source_value + target_value;
							moves.push(Move::Merge { from:        self.cursor.source,
							                         to:          self.cursor.target,
							                         start_value: source_value,
							                         end_value:   merged_value });
							self.cursor.move_to_target(merged_value);
							if self.cursor.advance_both().is_err() {
								break;
							};
						} else {
							if !self.cursor.target_changed {
								moves.push(Move::Stay { at: self.cursor.target, value: target_value });
							}
							if self.cursor.advance_target().is_err() {
								// reached end of board while target has a value
								moves.push(Move::Stay { at: self.cursor.source, value: source_value });
								break;
							};
						},
				}
			}
		}
		moves
	}
}
