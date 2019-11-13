use std::cell::RefCell;
use std::rc::Rc;

use controllers::{ConsoleController, Controller, NCursesController};
use game::Game;
use views::{ConsoleView, NCursesView};

mod controllers;
mod game;
mod views;

pub enum ViewType {
	Console,
	NCurses
}

pub fn play(view_type: ViewType, size_x: usize, size_y: usize) {
	let game = Rc::new(RefCell::new(Game::new(size_x, size_y)));
	let controller: Box<dyn Controller>;

	match view_type {
		ViewType::Console => {
			let view = ConsoleView::new(Rc::clone(&game));
			controller = Box::new(ConsoleController::create(Rc::clone(&game), view));
		},
		ViewType::NCurses => {
			let view = NCursesView::new(Rc::clone(&game));
			controller = Box::new(NCursesController::create(Rc::clone(&game), view));
		}
	}

	controller.run_game();
}
