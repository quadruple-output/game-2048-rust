use ncurses as nc;
use std::cell::RefCell;
use std::rc::Rc;

use super::View;
use crate::game::{Game, Square};

// NCurses HOWTO: http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/
// man pages: man 3x <function>
//

pub struct NCursesView {
	game: Rc<RefCell<Game>>
}

impl View for NCursesView {
	fn update(&self) {
		nc::erase(); // like clear(), but without implicit refresh()
		let board_box_window = self.position_board_box_in(nc::stdscr());
		// 		if let GameState::Over = game.state() {
		// 			nc::wattr_on(board_win, nc::A_BLINK());
		// 		}
		let board_window = self.boxed_subwindow(board_box_window);
		// 		nc::wattr_off(board_win, nc::A_BLINK());
		// 		if let GameState::Over = game.state() {
		// 			nc::wattr_on(board_win, nc::A_STANDOUT());
		// 		}
		self.show_board_in_window(board_window);
		// 		nc::wattr_off(board_win, nc::A_STANDOUT());
		nc::refresh();
	}
}

impl NCursesView {
	const BORDER_WIDTH: i32 = 1;

	pub fn new(game: Rc<RefCell<Game>>) -> Self {
		nc::initscr();
		nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
		nc::refresh(); // required for first wrefresh to work
		NCursesView { game }
	}

	fn position_board_box_in(&self, window: nc::WINDOW) -> nc::WINDOW {
		let (mut screen_height, mut screen_width) = window.size();
		// leave room for outer box:
		screen_height -= 2 * Self::BORDER_WIDTH;
		screen_width -= 2 * Self::BORDER_WIDTH;
		// calc optimal coords for board:
		let (squares_height, squares_width) = self.calc_optimal_board_win(screen_height, screen_width);
		// re-apply room for outer box:
		nc::derwin(window, squares_height + 2 * Self::BORDER_WIDTH, squares_width + 2 * Self::BORDER_WIDTH, 0, 0)
	}

	fn boxed_subwindow(&self, window: nc::WINDOW) -> nc::WINDOW {
		nc::box_(window, 0, 0);
		let (win_height, win_width) = window.size();
		nc::derwin(
		           window,
		           win_height - 2 * Self::BORDER_WIDTH,
		           win_width - 2 * Self::BORDER_WIDTH,
		           Self::BORDER_WIDTH,
		           Self::BORDER_WIDTH
		)
	}

	fn calc_optimal_board_win(&self, max_height: i32, max_width: i32) -> (i32, i32) {
		let board = &self.game.borrow().board;
		// calculate dimensions such that dimension % game.size == 0
		let mut height_mod_game_size = max_height - (max_height % board.size_y() as i32);
		let width_mod_game_size = max_width - (max_width % board.size_x() as i32);
		// height must be a multiple of an odd number, in order to vertically center the
		// label:
		if height_mod_game_size % 2 == 0 {
			height_mod_game_size -= board.size_y() as i32;
		}
		// minimum height is one row per square:
		if height_mod_game_size < board.size_y() as i32 {
			height_mod_game_size = board.size_y() as i32;
		}
		(height_mod_game_size, width_mod_game_size)
	}

	fn show_board_in_window(&self, window: nc::WINDOW) {
		let board = &self.game.borrow().board;

		let size_x = board.size_x();
		let size_y = board.size_y();
		for x in 0..size_x {
			for y in 0..size_y {
				let square_window = self.position_square_in(x, y, window);
				Self::show_square_in_window(&board.at_xy(x, y), square_window);
			}
		}
	}

	fn position_square_in(&self, square_x: usize, square_y: usize, window: nc::WINDOW) -> nc::WINDOW {
		//(y as i32 * win_height + win_height / 2) / size_y as i32,
		// 			              (x as i32 * win_width + win_width / 2) / size_x as i32,
		let (win_height, win_width) = window.size();
		let board = &self.game.borrow().board;
		let top = (square_y as i32 * win_height) / board.size_y() as i32;
		let bottom = ((square_y as i32 + 1) * win_height) / board.size_y() as i32;
		let left = (square_x as i32 * win_width) / board.size_x() as i32;
		let right = ((square_x as i32 + 1) * win_width) / board.size_x() as i32;
		nc::derwin(window, bottom - top, right - left, top, left)
	}

	fn show_square_in_window(square: &Square, window: nc::WINDOW) {
		let label = match square {
			Square::Empty => String::new(),
			Square::Value(value) => value.to_string()
		};
		nc::werase(window);
		let (win_height, win_width) = window.size();
		if win_height >= 3 && win_width >= 6 {
			// enough room for a box
			nc::box_(window, 0, 0);
		}
		// no room for a box around each square
		nc::mvwaddstr(window, win_height / 2, (win_width - label.len() as i32) / 2, &label);
	}
}

trait EnhancedWindow {
	fn size(&self) -> (i32, i32);
	fn drop(&mut self);
}

impl EnhancedWindow for nc::WINDOW {
	fn size(&self) -> (i32, i32) {
		let (mut win_height, mut win_width) = (0, 0);
		nc::getmaxyx(*self, &mut win_height, &mut win_width);
		(win_height, win_width)
	}

	fn drop(&mut self) { nc::delwin(*self); }
}

impl Drop for NCursesView {
	fn drop(&mut self) {
		nc::endwin();
		println!("fin de ncurses.");
		// TODO: register endwin on CTRL-C
	}
}
