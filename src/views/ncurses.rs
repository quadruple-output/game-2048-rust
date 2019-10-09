use ncurses as nc;
use std::cell::RefCell;
use std::rc::Rc;

use super::View;
use crate::game::{Board, Game, GameState, Square};

// NCurses HOWTO: http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/
// man pages: man 3x <function>
//

pub struct NCursesView {
	game: Rc<RefCell<Game>>
}

impl View for NCursesView {
	fn update(&self) {
		let game = self.game.borrow();
		let (mut screen_height, mut screen_width) = (0, 0);
		let (mut win_height, mut win_width);
		nc::erase(); // like clear(), but without implicit refresh()
		nc::getmaxyx(nc::stdscr(), &mut screen_height, &mut screen_width);
		win_height = screen_height - 4;
		win_width = screen_width - 4;
		// calculate height of window for optimal symmetry (height-2)%game.size == 0
		win_height = win_height - (win_height - 2) % game.board.size_y() as i32; // -2 for borders
		win_width = win_width - (win_width - 2) % game.board.size_x() as i32;
		let board_win = nc::subwin(nc::stdscr(), win_height, win_width, 2, 2);
		if let GameState::Over = game.state() {
			nc::wattr_on(board_win, nc::A_BLINK());
		}
		nc::box_(board_win, 0, 0);
		nc::wattr_off(board_win, nc::A_BLINK());
		if let GameState::Over = game.state() {
			nc::wattr_on(board_win, nc::A_STANDOUT());
		}
		self.show_board_in_window(&game.board, board_win);
		nc::wattr_off(board_win, nc::A_STANDOUT());
		nc::refresh();
		nc::delwin(board_win);
	}
}

impl NCursesView {
	pub fn new(game: Rc<RefCell<Game>>) -> Self {
		nc::initscr();
		nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
		nc::refresh(); // required for first wrefresh to work
		NCursesView { game }
	}

	fn show_board_in_window(&self, board: &Board, window: nc::WINDOW) {
		let mut win_width = 0;
		let mut win_height = 0;
		nc::getmaxyx(window, &mut win_height, &mut win_width);
		win_height -= 2; // -2 chars for border
		win_width -= 2; // -2 chars for border
		let size_x = board.size_x();
		let size_y = board.size_y();
		for x in 0..size_x {
			for y in 0..size_y {
				nc::wmove(
				          window,
				          1 + ((y as i32 * win_height + win_height / 2) / size_y as i32),
				          1 + ((x as i32 * win_width + win_width / 2) / size_x as i32)
				);
				let label = match board.at_xy(x, y) {
					Square::Empty => String::new(),
					Square::Value(value) => value.to_string()
				};
				nc::waddstr(window, &label);
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
