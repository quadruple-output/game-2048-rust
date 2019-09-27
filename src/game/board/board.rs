use super::coord::{Coord, Vector};
use super::cursor::DualCursor;
use super::{Square, Square::*};
use rand::{distributions::IndependentSample, Rng};

#[derive(Clone, Debug)]
pub struct Board {
    pub size: usize, // used as array index -> must be typed 'usize'
    pub grid: Vec<Vec<Square>>,
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
            rand_range_10: rand::distributions::Range::new(0, 10),
            rng: rand::thread_rng(),
        }
    }

    pub fn coord(&self, x: usize, y: usize) -> Coord {
        Coord::new(x, y, self.size)
    }

    pub fn initialize(&mut self) {
        self.grid = vec![vec![Square::Empty; self.size]; self.size];
        self.new_tile().unwrap();
    }

    pub fn new_tile(&mut self) -> Result<(), ()> {
        let n = self.rng.gen_range(0, self.num_free_tiles()?);
        let rnd_free_coord = self.find_free_tile(n);
        let new_value = if self.ten_percent_chance() { 4 } else { 2 };
        self.put(rnd_free_coord, Value(new_value));
        Ok(())
    }

    fn num_free_tiles(&self) -> Result<usize, ()> {
        let mut n = 0;
        for column in &self.grid {
            for square in column {
                if let Empty = square {
                    n += 1;
                }
            }
        }
        if n == 0 {
            Err(())
        } else {
            Ok(n)
        }
    }

    fn find_free_tile(&self, n: usize) -> Coord {
        let mut count = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                if let Empty = self.grid[x][y] {
                    if count == n {
                        return self.coord(x, y);
                    }
                    count += 1;
                }
            }
        }
        panic!();
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