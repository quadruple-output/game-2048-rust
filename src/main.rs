mod controllers;
mod game;
mod views;

use controllers::ConsoleController;
use game::Board;
use game::GameState;
use views::ConsoleView;

fn main() {
    let mut board = Board::new();
    let view = ConsoleView::new();
    let controller = ConsoleController::new();

    board.new_tile();
    loop {
        view.show(&board);
        match controller.effectuate(&mut board) {
            GameState::Finished => break,
            GameState::Running => (),
        }
    }
}
