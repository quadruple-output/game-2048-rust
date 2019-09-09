use crate::game::Board;

pub struct View {}

impl View {
    pub fn new() -> View {
        View {}
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
