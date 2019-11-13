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

pub fn run(view_type: ViewType) {
	let game = Rc::new(RefCell::new(Game::new(5, 5)));
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
