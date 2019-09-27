use std::cell::RefCell;
use std::rc::Rc;

use controllers::{ConsoleController, Controller, NCursesController};
use game::Game;
use views::{ConsoleView, NCursesView};

mod controllers;
mod game;
mod views;

fn main() {
    let game = Rc::new(RefCell::new(Game::new()));
    let view = ConsoleView::new(Rc::clone(&game));
    let controller = ConsoleController::new(Rc::clone(&game), view);

    controller.run_game();
}
