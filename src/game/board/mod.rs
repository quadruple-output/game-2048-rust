#[cfg(test)]
mod test;

use rand::distributions::IndependentSample;

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views/console.rs, if possible
#[derive(PartialEq)] // needed for tests
pub enum Square {
    Empty,
    Value(u16),
}

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
                    if self.at(Coord { x, y }) != other.at(Coord { x, y }) {
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

    fn put(&mut self, coord: Coord, square: Square) {
        self.grid[coord.x][coord.y] = square;
    }

    fn ten_percent_chance(&mut self) -> bool {
        self.rand_range_10.ind_sample(&mut self.rng) == 0
    }

    fn random_grid_size(&mut self) -> usize {
        self.rand_range_grid.ind_sample(&mut self.rng)
    }

    pub fn shift_left(&mut self) {
        for y in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new(0, y, 1, 0, self.size - 1, y));
        }
    }

    pub fn shift_right(&mut self) {
        for y in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new(self.size - 1, y, -1, 0, 0, y));
        }
    }

    pub fn shift_up(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new(x, 0, 0, 1, x, self.size - 1));
        }
    }

    pub fn shift_down(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new(x, self.size - 1, 0, -1, x, 0));
            // self.contract( Coord{x,self.size-1}, XYVector{dx:0,dy:-1}, Coord{x,0});
        }
    }

    fn condense(&mut self, stepper: &mut Stepper) {
        let mut target = stepper.position();
        let mut inspected;
        while !stepper.finished() {
            stepper.advance();
            inspected = stepper.position();
            match self.at(target) {
                Square::Empty => match self.at(inspected) {
                    Square::Empty => {} // just advance
                    Square::Value(v_inspect) => {
                        self.put(target, Square::Value(v_inspect));
                        self.put(inspected, Square::Empty);
                        // and advance
                    }
                },
                Square::Value(v_target) => match self.at(inspected) {
                    Square::Empty => {} // just advance
                    Square::Value(v_inspect) => {
                        if v_target == v_inspect {
                            self.put(target, Square::Value(v_target + v_inspect));
                            self.put(inspected, Square::Empty);
                        } else {
                            // TODO: move inspected adjacent to target
                        }
                        stepper.advance_position(&mut target);
                        // and advance inspected
                    }
                },
            }
        }
        // TODO: write some tests
    }
}

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

struct XYVector {
    dx: isize,
    dy: isize,
}

impl std::cmp::PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Stepper {
    position: Coord,
    step: XYVector,
    end: Coord,
}

impl Stepper {
    fn new(
        start_x: usize,
        start_y: usize,
        step_x: isize,
        step_y: isize,
        end_x: usize,
        end_y: usize,
    ) -> Stepper {
        Stepper {
            position: Coord {
                x: start_x,
                y: start_y,
            },
            step: XYVector {
                dx: step_x,
                dy: step_y,
            },
            end: Coord { x: end_x, y: end_y },
        }
    }
    fn finished(&self) -> bool {
        self.position == self.end
    }
    fn position(&self) -> Coord {
        self.position
    }
    fn advance(&mut self) {
        let mut pos = self.position;
        self.advance_position(&mut pos); // TODO: learn how to NOT use a temp var here
        self.position = pos;
    }
    fn advance_position(&self, position: &mut Coord) {
        position.x = (position.x as isize + self.step.dx) as usize;
        position.y = (position.y as isize + self.step.dy) as usize;
    }
}
