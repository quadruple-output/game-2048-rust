use super::coord::{Coord, Vector};
use super::cursor::DualCursor;
use super::{Square, Square::*};
use rand::distributions::IndependentSample;

#[derive(Clone, Debug)]
pub struct Board {
    pub size: usize, // used as array index -> must be typed 'usize'
    pub grid: Vec<Vec<Square>>,
    rand_range_grid: rand::distributions::Range<usize>, // array indexes must be typed 'usize'
    rand_range_10: rand::distributions::Range<u8>,
    rng: rand::ThreadRng,
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        if self.size != other.size {
            false
        } else {
            for x in 0..self.size {
                for y in 0..self.size {
                    if self.grid[x][y] != other.grid[x][y] {
                        return false;
                    }
                }
            }
            true
        }
    }
}

impl Board {
    pub fn new() -> Board {
        let size = 4;
        Board {
            size: size, // TODO: to make this a variable, the type of 'grid' needs to be non-array
            grid: vec![vec![Square::Empty; size]; size],
            rand_range_grid: rand::distributions::Range::new(0, size),
            rand_range_10: rand::distributions::Range::new(0, 10),
            rng: rand::thread_rng(),
        }
    }

    pub fn coord(&self, x: usize, y: usize) -> Coord {
        Coord::new(x, y, self.size)
    }

    pub fn initialize(&mut self) {
        self.grid = vec![vec![Square::Empty; self.size]; self.size];
        self.new_tile();
    }

    pub fn new_tile(&mut self) {
        let x = self.random_grid_size();
        let y = self.random_grid_size();
        // TODO: avoid non-empty squares
        self.grid[x][y] = Square::Value(if self.ten_percent_chance() { 4 } else { 2 });
    }

    pub fn at(&self, coord: Coord) -> Square {
        self.grid[coord.x][coord.y]
    }

    pub fn put(&mut self, coord: Coord, square: Square) {
        self.grid[coord.x][coord.y] = square;
    }

    fn ten_percent_chance(&mut self) -> bool {
        self.rand_range_10.ind_sample(&mut self.rng) == 0
    }

    fn random_grid_size(&mut self) -> usize {
        self.rand_range_grid.ind_sample(&mut self.rng)
    }

    #[allow(unused_must_use)]
    pub fn shift_left(&mut self) {
        for y in 0..self.size {
            // todo: parallel execution
            self.contract(self.coord(0, y), Vector { dx: 1, dy: 0 });
        }
    }

    #[allow(unused_must_use)]
    pub fn shift_right(&mut self) {
        for y in 0..self.size {
            // todo: parallel execution
            self.contract(self.coord(self.size - 1, y), Vector { dx: -1, dy: 0 });
        }
    }

    #[allow(unused_must_use)]
    pub fn shift_down(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.contract(self.coord(x, self.size - 1), Vector { dx: 0, dy: -1 });
        }
    }

    #[allow(unused_must_use)]
    pub fn shift_up(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.contract(self.coord(x, 0), Vector { dx: 0, dy: 1 });
        }
    }

    fn contract(&mut self, start: Coord, direction: Vector) -> Result<(), ()> {
        let mut cursor = DualCursor::new(start, direction);
        loop {
            match self.at(cursor.source) {
                Empty => {
                    cursor.advance_source()?;
                }
                Value(source_value) => match self.at(cursor.target) {
                    Empty => {
                        self.put(cursor.target, Value(source_value));
                        self.put(cursor.source, Empty);
                        cursor.advance_source()?;
                    }
                    Value(target_value) => {
                        if target_value == source_value {
                            self.put(cursor.target, Value(source_value + target_value));
                            self.put(cursor.source, Empty);
                            cursor.advance_both()?;
                        } else {
                            cursor.advance_target()?;
                        }
                    }
                },
            }
        }
    }
}
