use ::std::cell::RefCell;
use ::std::rc::Rc;

use super::View;
use crate::game::{Board, Game, Square};

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
    pub fn new(game: Rc<RefCell<Game>>) -> ConsoleView {
        ConsoleView { game }
    }

    fn show_board(&self, board: &Board) {
        for y in 0..4 {
            for x in 0..4 {
                match board.grid[x][y] {
                    Square::Empty => print!("[     ]"),
                    Square::Value(v) => print!("[{0:^5}]", v),
                }
            }
            println!("");
        }
    }
}
