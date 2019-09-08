use rand::Rng;

pub struct Board {
    pub grid: [[Square; 4]; 4],
}

#[derive(Copy, Clone, Debug)]
pub enum Square {
    Empty,
    Value(u16),
}


impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Square::Empty; 4]; 4],
        }
    }

    pub fn restart(&mut self) {
        self.grid = [[Square::Empty; 4]; 4];
    }

    pub fn new_tile(&mut self) {
        let x = rand::thread_rng().gen_range(0, 4);
        let y = rand::thread_rng().gen_range(0, 4);
        self.grid[x][y] = Square::Value(2);
    }
}
