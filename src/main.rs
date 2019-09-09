mod controllers;
mod game;
mod views;

use game::GameState::*; // Finished and Running

fn main() {
    let mut game = game::Game::new();
    let view = views::ConsoleView::new();
    let controller = controllers::ConsoleController::new();

    loop {
        view.show(&game);
        game.execute(&controller.receive_command());
        match game.state() {
            Finished => break,
            Running => (),
        }
    }
}
