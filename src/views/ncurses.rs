use crate::game::{Board, Game, Square};
use ncurses::*;

//
// NCurses HOWTO: http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/
//

#[allow(dead_code)]
pub struct View {}

impl View {
    #[allow(dead_code)]
    pub fn new() -> View {
        initscr();
        View {}
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn show(&self, game: &Game) {
        self.show_board(&game.board);
        refresh();
    }

    fn show_board(&self, board: &Board) {
        for x in 0..board.size {
            for y in 0..board.size {
                mv(y as i32, (x * 6) as i32);
                let label = match board.grid[x][y] {
                    Square::Empty => String::new(),
                    Square::Value(value) => value.to_string(),
                };
                addstr(&label);
            }
            addstr("");
        }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        endwin();
        println!("fin.");
        // TODO: register endwin on CTRL-C
    }
}
