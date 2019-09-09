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
        let command = controller.receive_command();
        game.execute_command(&command); // TODO: should be 'game'
        match game.state() {
            // TODO: should be 'game'
            Finished => break,
            Running => (),
        }
    }
}
