use std::cell::RefCell;

use controllers::{ConsoleController, Controller, NCursesController};
use game::Game;
use views::{ConsoleView, NCursesView};

mod controllers;
mod game;
mod views;

pub enum ViewType {
  Console,
  NCurses,
}

pub fn play(view_type: ViewType, size_x: usize, size_y: usize) {
  let game = RefCell::new(Game::new(size_x, size_y));

  let controller: Box<dyn Controller> = match view_type {
    ViewType::Console => {
      let view = ConsoleView::new(&game);
      Box::new(ConsoleController::create(&game, view))
    },
    ViewType::NCurses => {
      let view = NCursesView::new(&game);
      Box::new(NCursesController::create(&game, view))
    },
  };

  controller.run_game();
}
