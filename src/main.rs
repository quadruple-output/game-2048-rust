mod controllers;
mod game;
mod views;

use game::GameState::*; // Finished and Running

fn main() {
    let mut board = game::Board::new();
    let view = views::NCursesView::new();
    let controller = controllers::NCursesController::new();

    controller.initialize_game(&mut board);
    loop {
        view.show(&board);
        match controller.effectuate(&mut board) {
            Finished => break,
            Running => (),
        }
    }
}
