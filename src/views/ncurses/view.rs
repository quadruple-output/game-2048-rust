use ncurses as nc;
use std::cell::RefCell;
use std::rc::Rc;

use super::animator::Animator;
use super::pallete::Pallete;

use crate::game::{Coord, Game, Move};
use crate::views::View;

// NCurses HOWTO: http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/
// man pages: man 3x <function>
//

pub struct NCursesView {
	game:     Rc<RefCell<Game>>,
	pallete:  Pallete,
	animator: Animator
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
		// 		nc::wattr_off(board_win, nc::A_STANDOUT());
		nc::wnoutrefresh(nc::stdscr());
		self.animator.animate(|t| {
			             nc::werase(board_window);
			             self.show_board_in_window(board_window, t);
			             nc::wnoutrefresh(board_window);
			             nc::doupdate();
		             });
	}
}

impl NCursesView {
	const BORDER_WIDTH: i32 = 1;

	pub fn new(game: Rc<RefCell<Game>>) -> Self {
		nc::initscr();
		nc::start_color();
		nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
		nc::refresh(); // required for first wrefresh to work
		NCursesView { game, pallete: Pallete::new(), animator: Animator::new(0.5, 10) }
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

	fn show_board_in_window(&self, window: nc::WINDOW, t: f32) {
		let game = self.game.borrow();

		for r#move in game.latest_moves().iter() {
			match r#move {
				Move::Appear { at, value } =>
					if t == 1.0 {
						let square_window = self.position_square_in(*at, *at, window, t);
						self.show_square_in_window(*value, square_window);
					},
				Move::Shift { from, to, value } => {
					let square_window = self.position_square_in(*from, *to, window, t);
					self.show_square_in_window(*value, square_window);
				},
				Move::Merge { from, to, start_value, end_value } =>
					if t == 1.0 {
						let square_window = self.position_square_in(*from, *to, window, t);
						self.show_square_in_window(*end_value, square_window);
					} else {
						let square_window_from = self.position_square_in(*from, *to, window, t);
						let square_window_to = self.position_square_in(*from, *to, window, 1.0);
						self.show_square_in_window(*start_value, square_window_to);
						self.show_square_in_window(*start_value, square_window_from);
					},
				Move::Stay { at, value } => {
					let square_window = self.position_square_in(*at, *at, window, t);
					self.show_square_in_window(*value, square_window);
				}
			}
		}
	}

	fn position_square_in(&self, start_coord: Coord, end_coord: Coord, window: nc::WINDOW, t: f32)
	                      -> nc::WINDOW {
		let (win_height, win_width) = window.size();
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
		nc::derwin(window, bottom - top, right - left, top, left)
	}

	pub fn interpolate(&self, a: i32, b: i32, t: f32) -> i32 { a + (t * (b as f32 - a as f32)) as i32 }

	fn show_square_in_window(&self, value: u16, window: nc::WINDOW) {
		let label = value.to_string();
		nc::wattr_set(window, 0, 2);
		nc::wbkgdset(window, self.pallete.get_pair_for_square_value(value));
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
