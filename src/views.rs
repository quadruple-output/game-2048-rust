use crate::game::Board;

pub struct ConsoleView {}

impl ConsoleView {
    pub fn new() -> ConsoleView {
        ConsoleView {}
    }

    pub fn show(&self, board: &Board) {
        for x in 0..4 {
            for y in 0..4 {
                print!(" {:?} ", board.grid[x][y]);
            }
            println!("");
        }
    }
}
