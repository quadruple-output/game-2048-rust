use crate::game::{Board, Game};

#[allow(dead_code)]
pub struct View {}

impl View {
    #[allow(dead_code)]
    pub fn new() -> View {
        View {}
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn show(&self, game: &Game) {
        self.show_board(&game.board);
    }

    fn show_board(&self, board: &Board) {}
}
