use ncurses as nc;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::Controller;
use crate::game::{Command, Game};
use crate::views::{NCursesView, View};

pub struct NCursesController {
	game: Rc<RefCell<Game>>,
	view: NCursesView
}

impl NCursesController {
	pub fn create(game: Rc<RefCell<Game>>, view: NCursesView) -> impl Controller {
		nc::cbreak();
		nc::keypad(nc::stdscr(), true);
		nc::noecho();
		NCursesController { game, view }
	}
}

impl Controller for NCursesController {
	fn view(&self) -> &dyn View { &self.view }

	fn game(&self) -> Ref<Game> { self.game.borrow() }

	fn mut_game(&self) -> RefMut<Game> { self.game.borrow_mut() }

	fn receive_command(&self) -> Command {
		loop {
			let key = nc::getch();
			match key {
				nc::KEY_RESIZE => break Command::Nop, // window resize event
				nc::KEY_LEFT => break Command::Left,
				nc::KEY_RIGHT => break Command::Right,
				nc::KEY_UP => break Command::Up,
				nc::KEY_DOWN => break Command::Down,
				_ => ()
			}
			match key as u8 as char {
				'n' | 'N' => break Command::New,
				'q' | 'Q' => break Command::Quit,
				_ => ()
			}
		}
	}
}
