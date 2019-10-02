use super::coord::{Coord, Vector};
use super::cursor::DualCursor;
use super::r#move::Move;
use super::{Square, Square::*};
use rand::{distributions::IndependentSample, Rng};

#[derive(Clone, Debug)]
pub struct Board {
    size: usize, // used as array index -> must be typed 'usize'
    grid: Vec<Vec<Square>>,
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
        self.new_tile();
    }

    pub fn new_tile(&mut self) {
        let num_free_tiles = self.num_free_tiles();
        if num_free_tiles == 0 {
            panic!("tried to place a new tile on a full board")
        };
        let n = self.rng.gen_range(0, num_free_tiles);
        let rnd_free_coord = self.find_free_tile(n);
        let new_value = if self.ten_percent_chance() { 4 } else { 2 };
        self.put(rnd_free_coord, Value(new_value));
    }

    fn num_free_tiles(&self) -> usize {
        let mut n = 0;
        for column in &self.grid {
            for square in column {
                if let Empty = square {
                    n += 1;
                }
            }
        }
        n
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

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn at(&self, coord: Coord) -> Square {
        self.at_xy(coord.x,coord.y)
    }

    pub fn at_xy(&self, x:usize,y:usize) -> Square {
        self.grid[x][y]
    }

    pub fn put(&mut self, coord: Coord, square: Square) {
        self.grid[coord.x][coord.y] = square;
    }

    fn ten_percent_chance(&mut self) -> bool {
        self.rand_range_10.ind_sample(&mut self.rng) == 0
    }

    pub fn shift_left(&mut self) -> Result<Vec<Move>, ()> {
        let mut moves = Vec::new();
        for y in 0..self.size {
            // todo: parallel execution
            moves.append(&mut self.contract(self.coord(0, y), Vector { dx: 1, dy: 0 }));
        }
        if moves.is_empty() {
            Err(())
        } else {
            Ok(moves)
        }
    }

    pub fn shift_right(&mut self) -> Result<Vec<Move>, ()> {
        let mut moves = Vec::new();
        for y in 0..self.size {
            // todo: parallel execution
            moves
                .append(&mut self.contract(self.coord(self.size - 1, y), Vector { dx: -1, dy: 0 }));
        }
        if moves.is_empty() {
            Err(())
        } else {
            Ok(moves)
        }
    }

    pub fn shift_down(&mut self) -> Result<Vec<Move>, ()> {
        let mut moves = Vec::new();
        for x in 0..self.size {
            // todo: parallel execution
            moves
                .append(&mut self.contract(self.coord(x, self.size - 1), Vector { dx: 0, dy: -1 }));
        }
        if moves.is_empty() {
            Err(())
        } else {
            Ok(moves)
        }
    }

    pub fn shift_up(&mut self) -> Result<Vec<Move>, ()> {
        let mut moves = Vec::new();
        for x in 0..self.size {
            // todo: parallel execution
            moves.append(&mut self.contract(self.coord(x, 0), Vector { dx: 0, dy: 1 }));
        }
        if moves.is_empty() {
            Err(())
        } else {
            Ok(moves)
        }
    }

    fn contract(&mut self, start: Coord, direction: Vector) -> Vec<Move> {
        let mut result = Vec::new();
        let mut cursor = DualCursor::new(start, direction);
        loop {
            match self.at(cursor.source) {
                Empty => {
                    if let Err(_) = cursor.advance_source() {
                        break;
                    };
                }
                Value(source_value) => match self.at(cursor.target) {
                    Empty => {
                        result.push(Move::new(
                            cursor.source,
                            cursor.target,
                            source_value,
                            source_value,
                            Empty,
                        ));
                        self.put(cursor.target, Value(source_value));
                        self.put(cursor.source, Empty);
                        if let Err(_) = cursor.advance_source() {
                            break;
                        }
                    }
                    Value(target_value) => {
                        if target_value == source_value {
                            result.push(Move::new(
                                cursor.source,
                                cursor.target,
                                source_value,
                                source_value + target_value,
                                Value(target_value),
                            ));
                            self.put(cursor.target, Value(source_value + target_value));
                            self.put(cursor.source, Empty);
                            if let Err(_) = cursor.advance_both() {
                                break;
                            };
                        } else {
                            if let Err(_) = cursor.advance_target() {
                                break;
                            };
                        }
                    }
                },
            }
        }
        result
    }
}
