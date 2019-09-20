#[cfg(test)]
mod test;

use self::Square::*;
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

    pub fn shift_down(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new(x, self.size - 1, 0, -1, x, 0));
        }
    }

    #[allow(unused_must_use)]
    pub fn shift_up(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            // self.condense(&mut Stepper::new(x, 0, 0, 1, x, self.size - 1));
            self.contract(Coord { x, y: 0 }, Vector { dx: 0, dy: 1 }, self.size);
        }
    }

    fn contract(&mut self, start: Coord, direction: Vector, num_steps: usize) -> Result<(), ()> {
        let mut cursor = DualCursor::new(start, direction, num_steps);
        loop {
            if let Value(source_value) = self.at(cursor.source) {
                match self.at(cursor.target) {
                    Empty => {
                        self.put(cursor.target, Value(source_value));
                        self.put(cursor.source, Empty)
                    }
                    Value(target_value) => {
                        if target_value == source_value {
                            self.put(cursor.target, Value(source_value + target_value));
                            self.put(cursor.source, Empty)
                        }
                        cursor.advance_target()?;
                    }
                };
            }
            cursor.advance_source()?;
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

impl std::cmp::PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coord {
    fn add(&self, vector: Vector) -> Result<Self, ()> {
        // TODO: enable use of '+' operator (not trivial with Rhs being of different type)
        let new_x = self.x as isize + vector.dx;
        let new_y = self.y as isize + vector.dy;
        if new_x < 0 || new_y < 0 {
            Err(())
        } else {
            Ok(Self {
                x: new_x as usize,
                y: new_y as usize,
            })
        }
    }
}

#[derive(Copy, Clone)]
struct Vector {
    dx: isize,
    dy: isize,
}

impl Vector {
    fn times(&self, factor: usize) -> Self {
        Self {
            dx: self.dx * factor as isize,
            dy: self.dy * factor as isize,
        }
    }
}

struct DualCursor {
    target: Coord,
    source: Coord,
    direction: Vector,
    end: Result<Coord, ()>,
}

impl DualCursor {
    fn new(start: Coord, direction: Vector, num_squares: usize) -> DualCursor {
        DualCursor {
            target: start,
            source: start.add(direction).unwrap(),
            direction,
            end: start.add(direction.times(num_squares)),
        }
    }

    fn advance_source(&mut self) -> Result<(), ()> {
        self.source = self.source.add(self.direction)?;
        if let Ok(end_coord) = self.end {
            // if self.end is not Ok, the 'add' method above raises an error at end
            if self.source == end_coord {
                return Err(());
            }
        }
        Ok(())
    }

    fn advance_target(&mut self) -> Result<(), ()> {
        self.target = self.target.add(self.direction)?;
        if let Ok(end_coord) = self.end {
            // if self.end is not Ok, the 'add' method above raises an error at end
            if self.target == end_coord {
                return Err(());
            }
        }
        Ok(())
    }
}

struct Stepper {
    position: Coord,
    step: Vector,
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
            step: Vector {
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
