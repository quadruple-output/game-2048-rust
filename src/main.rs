use std::io;

mod game;
mod views;
mod controllers;

use game::Board;
use game::GameState;
use views::ConsoleView;
use controllers::ConsoleController;

fn main() {
    let mut board = Board::new();
    let view = ConsoleView::new();
    let controller = ConsoleController::new();

    board.new_tile();
    loop {
        view.show_board(&board);
        match controller.effectuate(&mut board) {
            GameState::Finished => break,
            GameState::Running => (),
        }
    }
}

