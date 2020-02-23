use log::{debug, trace};
use ncurses as nc;
use std::cell::{Cell, RefCell};
use std::fmt;

use super::animator::Animator;
use super::pallete::Pallete;

use crate::game::{Coord, Game, Move};
use crate::views::View;

// NCurses HOWTO: http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/
// man pages: man 3x <function>
//

pub struct NCursesView<'a> {
	game:            &'a RefCell<Game>,
	pallete:         Pallete,
	animator:        Animator,
	last_shown_move: Cell<usize>,
	stdscr:          NCWindow
}

struct NCWindow(nc::WINDOW, String); // "Newtype" wrapper pattern for implementing Drop for nc::WINDOW

impl<'a> View for NCursesView<'a> {
	fn update(&self) {
		debug!("Start update view");
		nc::erase(); // like clear(), but without implicit refresh()
		let board_box_window = self.position_board_in(&self.stdscr);
		// TODO:
		// if let GameState::Over = game.state() {
		// 	nc::wattr_on(board_win, nc::A_BLINK());
		// }
		let board_window = self.boxed_subwindow(&board_box_window);
		// 		nc::wattr_off(board_win, nc::A_BLINK());
		// 		if let GameState::Over = game.state() {
		// 			nc::wattr_on(board_win, nc::A_STANDOUT());
		// 		}
		// 		nc::wattr_off(board_win, nc::A_STANDOUT());
		nc::wnoutrefresh(nc::stdscr());
		let draw_frame = |t| {
			debug!("Draw frame, t={:?}", t);
			nc::werase(board_window.0);
			self.show_board_in_window(&board_window, t);
			nc::wnoutrefresh(board_window.0);
			nc::doupdate();
		};
		if self.last_shown_move.get() == self.game.borrow().move_count() {
			// move has already been animated ⇒ only show last frame
			draw_frame(1.0);
		} else {
			debug!("Start animation");
			self.animator.animate(draw_frame);
			debug!("End animation");
			self.last_shown_move.set(self.game.borrow().move_count());
		}
		debug!("End update view");
	}
}

impl<'a> NCursesView<'a> {
	const BORDER_WIDTH: i32 = 1;

	pub fn new(game: &'a RefCell<Game>) -> Self {
		nc::initscr();
		nc::start_color();
		nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
		nc::refresh(); // required for first wrefresh to work
		let last_shown_move = game.borrow().move_count();
		NCursesView { game,
		              pallete: Pallete::new(),
		              animator: Animator::new(0.2, 50),
		              last_shown_move: Cell::new(last_shown_move),
		              stdscr: NCWindow::new(None, nc::stdscr(), "stdscr") }
	}

	fn position_board_in(&self, window: &NCWindow) -> NCWindow {
		let (mut screen_height, mut screen_width) = window.size();
		// leave room for outer box:
		screen_height -= 2 * Self::BORDER_WIDTH;
		screen_width -= 2 * Self::BORDER_WIDTH;
		// calc optimal coords for board:
		let (squares_height, squares_width) = self.calc_optimal_board_win(screen_height, screen_width);
		// re-apply room for outer box:
		NCWindow::new(
		              Some(window),
		              nc::derwin(
			window.0,
			squares_height + 2 * Self::BORDER_WIDTH,
			squares_width + 2 * Self::BORDER_WIDTH,
			0,
			0
		),
		              "board"
		)
	}

	fn boxed_subwindow(&self, window: &NCWindow) -> NCWindow {
		nc::box_(window.0, 0, 0);
		let (win_height, win_width) = window.size();
		NCWindow::new(
		              Some(window),
		              nc::derwin(
			window.0,
			win_height - 2 * Self::BORDER_WIDTH,
			win_width - 2 * Self::BORDER_WIDTH,
			Self::BORDER_WIDTH,
			Self::BORDER_WIDTH
		),
		              "boxed"
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

	#[allow(clippy::float_cmp)]
	fn show_board_in_window(&self, board_window: &NCWindow, t: f32) {
		let game = self.game.borrow();

		for r#move in game.latest_moves().iter() {
			match r#move {
				Move::Appear { at, value } =>
					if t == 1.0 {
						// last frame is guaranteed to be exactly 1.0 (=> clippy::float_cmp)
						let square_window = self.position_square_in(*at, *at, board_window, t);
						self.show_square_in_window(*value, &square_window);
					},
				Move::Shift { from, to, value } => {
					let square_window = self.position_square_in(*from, *to, board_window, t);
					self.show_square_in_window(*value, &square_window);
				},
				Move::Merge { from, to, start_value, end_value } => {
					let square_window = self.position_square_in(*from, *to, board_window, t);
					if t == 1.0 {
						// last frame is guaranteed to be exactly 1.0 (=> clippy::float_cmp)
						self.show_square_in_window(*end_value, &square_window);
					} else {
						self.show_square_in_window(*start_value, &square_window);
					}
				},
				Move::Stay { at, value } => {
					let square_window = self.position_square_in(*at, *at, board_window, t);
					self.show_square_in_window(*value, &square_window);
				}
			}
		}
	}

	fn position_square_in(&self, start_coord: Coord, end_coord: Coord, board_window: &NCWindow, t: f32)
	                      -> NCWindow {
		let (win_height, win_width) = board_window.size();
		let board = &self.game.borrow().board;
		let start_top = (start_coord.y as i32 * win_height) / board.size_y() as i32;
		let start_left = (start_coord.x as i32 * win_width) / board.size_x() as i32;
		let start_bottom = ((start_coord.y as i32 + 1) * win_height) / board.size_y() as i32;
		let start_right = ((start_coord.x as i32 + 1) * win_width) / board.size_x() as i32;
		let end_top = (end_coord.y as i32 * win_height) / board.size_y() as i32;
		let end_left = (end_coord.x as i32 * win_width) / board.size_x() as i32;
		let end_bottom = ((end_coord.y as i32 + 1) * win_height) / board.size_y() as i32;
		let end_right = ((end_coord.x as i32 + 1) * win_width) / board.size_x() as i32;
		let top = self.interpolate(start_top, end_top, t);
		let left = self.interpolate(start_left, end_left, t);
		let bottom = self.interpolate(start_bottom, end_bottom, t);
		let right = self.interpolate(start_right, end_right, t);
		NCWindow::new(
		              Some(board_window),
		              nc::derwin(board_window.0, bottom - top, right - left, top, left),
		              "tile"
		)
	}

	pub fn interpolate(&self, a: i32, b: i32, t: f32) -> i32 { a + (t * (b as f32 - a as f32)) as i32 }

	fn show_square_in_window(&self, value: u16, window: &NCWindow) {
		let label = value.to_string();
		nc::wattr_set(window.0, 0, 2);
		nc::wbkgdset(window.0, self.pallete.get_pair_for_square_value(value));
		nc::touchwin(window.0); // attempt to fix broken rendering. suggested in 'man 3x wrefresh' for window overlaps
		nc::werase(window.0);
		let (win_height, win_width) = window.size();
		if win_height >= 3 && win_width >= 6 {
			// enough room for a box
			nc::box_(window.0, 0, 0);
		}
		// no room for a box around each square
		nc::mvwaddstr(window.0, win_height / 2, (win_width - label.len() as i32) / 2, &label);
	}
}

impl NCWindow {
	fn new(parent: Option<&NCWindow>, wrappee: nc::WINDOW, label: &str) -> Self {
		let mut new_label = String::new();
		if let Some(parent) = parent {
			new_label.push_str(&parent.1);
			new_label.push_str("::");
		};
		new_label.push_str(label);
		let new_win = NCWindow(wrappee, new_label);
		trace!("new Window {:?}", new_win);
		new_win
	}

	fn size(&self) -> (i32, i32) {
		let (mut win_height, mut win_width) = (0, 0);
		nc::getmaxyx(self.0, &mut win_height, &mut win_width);
		(win_height, win_width)
	}
}

impl fmt::Debug for NCWindow {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let (height, width) = self.size();
		let (mut x, mut y) = (0, 0);
		nc::getparyx(self.0, &mut y, &mut x);
		write!(f, "{:?}: '{}' {}x{}@{},{}", self.0, self.1, width, height, x, y)
		// f.debug_struct("NCWindow").field("x", &self.x).field("y",
		// &self.y).finish()
	}
}

impl Drop for NCWindow {
	fn drop(&mut self) {
		// delwin is important to clear up ncurses. Otherwise, a screen redraw on window
		// resize takes more and more time the longer you play.
		trace!("drop Window: {:?}", self.0);
		assert!(nc::ERR != nc::delwin(self.0));
	}
}

impl<'a> Drop for NCursesView<'a> {
	fn drop(&mut self) {
		// reset terminal properties (e.g. make cursor visible again)
		nc::endwin(); // this also clears the screen and destroys any error output
		debug!("fin de ncurses.");
		// TODO: register endwin on CTRL-C
	}
}
