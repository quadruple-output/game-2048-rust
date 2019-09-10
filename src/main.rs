mod controllers;
mod game;
mod views;

use game::{Game, GameState::*}; // '::Finished' and '::Running'

fn main() {
    let mut game = Game::new();
    // let view = views::ConsoleView::new();
    // let controller = controllers::ConsoleController::new();
    let view = views::NCursesView::new();
    let controller = controllers::NCursesController::new();

    loop {
        view.show(&game);
        game.execute(&controller.receive_command());
        match game.state() {
            Finished => break,
            Running => (),
        }
    }
}
