use std::cell::RefCell;
use std::rc::Rc;

use controllers::{ConsoleController, Controller, NCursesController};
use game::{Game, GameState::Finished};
use views::{ConsoleView, NCursesView, View};

mod controllers;
mod game;
mod views;

fn main() {
    let game = Rc::new(RefCell::new(Game::new()));
    let view = NCursesView::new(Rc::clone(&game));
    let controller = NCursesController::new(Rc::clone(&game));

    loop {
        view.update();
        (*game).borrow_mut().execute(&controller.receive_command());
        if let Finished = (*game).borrow().state() {
            break;
        }
    }
}
