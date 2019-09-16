mod controllers;
mod game;
mod views;

use controllers::Controller;
use game::{Game, GameState::Finished};
use views::View;

fn main() {
    let mut game = Game::new();
    // let view = views::ConsoleView::new();
    // let controller = controllers::ConsoleController::new();
    let view = views::NCursesView::new();
    let controller = controllers::NCursesController::new();

    loop {
        view.show(&game);
        game.execute(&controller.receive_command());
        if let Finished = game.state() {
            break;
        }
    }
}
