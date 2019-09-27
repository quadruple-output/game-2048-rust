use::std::rc::Rc;
use::std::cell::RefCell;

use super::View;
use crate::game::{Board, Game};

#[allow(dead_code)]
pub struct ConsoleView {
    game: Rc<RefCell<Game>>,
}

impl View for ConsoleView {
    fn update(&self) {
        self.show_board(&self.game.borrow().board);
    }
}

impl ConsoleView {
    #[allow(dead_code)]
    pub fn new(game: Rc<RefCell<Game>>) -> impl View {
        ConsoleView { game }
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
