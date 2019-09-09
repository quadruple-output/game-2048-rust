use crate::game::Board;

#[allow(dead_code)]
pub struct View {}

impl View {
    #[allow(dead_code)]
    pub fn new() -> View {
        View {}
    }

    #[allow(dead_code)]
    pub fn show(&self, board: &Board) {
        for x in 0..4 {
            for y in 0..4 {
                print!(" {:?} ", board.grid[x][y]);
            }
            println!("");
        }
    }
}
