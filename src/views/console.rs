use ::std::cell::RefCell;

use super::View;
use crate::game::{Board, Game, Square};

pub struct ConsoleView<'a> {
	game: &'a RefCell<Game>
}

impl<'a> View for ConsoleView<'a> {
	fn update(&self) { self.show_board(&self.game.borrow().board); }
}

impl<'a> ConsoleView<'a> {
	pub fn new(game: &RefCell<Game>) -> ConsoleView { ConsoleView { game } }

	fn show_board(&self, board: &Board) {
		for y in 0..board.size_y() {
			for x in 0..board.size_x() {
				match board.at_xy(x, y) {
					Square::Empty => print!("[     ]"),
					Square::Value(v) => print!("[{0:^5}]", v)
				}
			}
			println!();
		}
	}
}
