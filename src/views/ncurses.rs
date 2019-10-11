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
		let board_box_window = self.fit_board_box_in(nc::stdscr());
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
		nc::delwin(board_window);
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

	fn fit_board_box_in(&self, window: nc::WINDOW) -> nc::WINDOW {
		let (mut screen_height, mut screen_width) = (0, 0);
		nc::getmaxyx(window, &mut screen_height, &mut screen_width);
		// leave room for outer box:
		screen_height -= 2 * Self::BORDER_WIDTH;
		screen_width -= 2 * Self::BORDER_WIDTH;
		// calc optimal coords for board:
		let (squares_height, squares_width) = self.calc_optimal_board_win(screen_height, screen_width);
		nc::derwin(window, squares_height + 2 * Self::BORDER_WIDTH, squares_width + 2 * Self::BORDER_WIDTH, 0, 0)
	}

	fn boxed_subwindow(&self, window: nc::WINDOW) -> nc::WINDOW {
		let (mut win_height, mut win_width) = (0, 0);
		nc::getmaxyx(window, &mut win_height, &mut win_width);
		nc::box_(window, 0, 0);
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

		let mut win_width = 0;
		let mut win_height = 0;
		nc::getmaxyx(window, &mut win_height, &mut win_width);
		let size_x = board.size_x();
		let size_y = board.size_y();
		for x in 0..size_x {
			for y in 0..size_y {
				let label = match board.at_xy(x, y) {
					Square::Empty => String::new(),
					Square::Value(value) => value.to_string()
				};
				nc::mvwaddstr(
				              window,
				              (y as i32 * win_height + win_height / 2) / size_y as i32,
				              (x as i32 * win_width + win_width / 2) / size_x as i32,
				              &label
				);
			}
		}
	}
}

impl Drop for NCursesView {
	fn drop(&mut self) {
		nc::endwin();
		println!("fin de ncurses.");
		// TODO: register endwin on CTRL-C
	}
}
