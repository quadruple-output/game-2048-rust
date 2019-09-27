use std::cell::RefCell;
use std::rc::Rc;

use controllers::Controller;
use game::{Game, GameState::Finished};
use views::View;

mod controllers;
mod game;
mod views;

fn main() {
    let game = Rc::new(RefCell::new(Game::new()));
    // let view = views::ConsoleView::new();
    // let controller = controllers::ConsoleController::new();
    let view = views::NCursesView::new(Rc::clone(&game));
    let controller = controllers::NCursesController::new(Rc::clone(&game));

    loop {
        view.show(&game.borrow());
        (*game).borrow_mut().execute(&controller.receive_command());
        if let Finished = (*game).borrow().state() {
            break;
        }
    }
}
