use ::std::cell::RefCell;
use ::std::rc::Rc;

use super::View;
use crate::game::{Board, Game, Square};

pub struct ConsoleView {
	game: Rc<RefCell<Game>>
}

impl View for ConsoleView {
	fn update(&self) { self.show_board(&self.game.borrow().board); }
}

impl ConsoleView {
	pub fn new(game: Rc<RefCell<Game>>) -> ConsoleView { ConsoleView { game } }

	fn show_board(&self, board: &Board) {
		for y in 0..board.size_y() {
			for x in 0..board.size_x() {
				match board.at_xy(x, y) {
					Square::Empty => print!("[     ]"),
					Square::Value(v) => print!("[{0:^5}]", v)
				}
			}
			println!("");
		}
	}
}
