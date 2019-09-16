use super::View;
use crate::game::{Board, Game};

#[allow(dead_code)]
pub struct ConsoleView {}

impl View for ConsoleView {
    fn show(&self, game: &Game) {
        self.show_board(&game.board);
    }
}

impl ConsoleView {
    #[allow(dead_code)]
    pub fn new() -> impl View {
        ConsoleView {}
    }

    fn show_board(&self, board: &Board) {
        for x in 0..4 {
            for y in 0..4 {
                print!(" {:?} ", board.grid[x][y]);
            }
            println!("");
        }
    }
}
