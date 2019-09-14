use rand::distributions::IndependentSample;

pub struct Game {
    pub board: Board,
    state: GameState,
}

pub enum Command {
    Nop, // no operation. Used for repainting
    New,
    Quit,
    Right,
    Left,
    Up,
    Down,
}

pub enum GameState {
    Running,
    Finished,
}

pub struct Board {
    pub size: usize, // used as array index -> must be typed 'usize'
    pub grid: Vec<Vec<Square>>,
    rand_range_grid: rand::distributions::Range<usize>, // array indexes must be typed 'usize'
    rand_range_10: rand::distributions::Range<u8>,
    rng: rand::ThreadRng,
}

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views/console.rs, if possible
pub enum Square {
    Empty,
    Value(u16),
}

impl Board {
    fn new() -> Board {
        let size = 4;
        Board {
            size: size, // TODO: to make this a variable, the type of 'grid' needs to be non-array
            grid: vec![vec![Square::Empty; size]; size],
            rand_range_grid: rand::distributions::Range::new(0, size),
            rand_range_10: rand::distributions::Range::new(0, 10),
            rng: rand::thread_rng(),
        }
    }

    fn initialize(&mut self) {
        self.grid = vec![vec![Square::Empty; self.size]; self.size];
        self.new_tile();
    }

    fn new_tile(&mut self) {
        let x = self.random_grid_size();
        let y = self.random_grid_size();
        // TODO: avoid non-empty squares
        self.grid[x][y] = Square::Value(if self.ten_percent_chance() { 4 } else { 2 });
    }

    fn ten_percent_chance(&mut self) -> bool {
        self.rand_range_10.ind_sample(&mut self.rng) == 0
    }

    fn random_grid_size(&mut self) -> usize {
        self.rand_range_grid.ind_sample(&mut self.rng)
    }

    fn shift_left(&mut self) {
        for y in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new([0, y], [1, 0], [self.size - 1, y]));
        }
    }

    fn shift_right(&mut self) {
        for y in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new([self.size - 1, y], [-1, 0], [0, y]));
        }
    }

    fn shift_up(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new([x, 0], [0, 1], [x, self.size - 1]));
        }
    }

    fn shift_down(&mut self) {
        for x in 0..self.size {
            // todo: parallel execution
            self.condense(&mut Stepper::new([x, self.size - 1], [0, -1], [x, 0]));
        }
    }

    fn condense(&mut self, stepper: &mut Stepper) {
        let mut target = stepper.position();
        let mut inspected;
        while !stepper.finished() {
            stepper.advance();
            inspected = stepper.position();
            match self.grid[target[0]][target[1]] {
                Square::Empty => match self.grid[inspected[0]][inspected[1]] {
                    Square::Empty => {} // just advance
                    Square::Value(v_inspect) => {
                        self.grid[target[0]][target[1]] = Square::Value(v_inspect);
                        self.grid[inspected[0]][inspected[1]] = Square::Empty;
                        // and advance
                    }
                },
                Square::Value(v_target) => match self.grid[inspected[0]][inspected[1]] {
                    Square::Empty => {} // just advance
                    Square::Value(v_inspect) => {
                        if v_target == v_inspect {
                            self.grid[target[0]][target[1]] = Square::Value(v_target + v_inspect);
                            self.grid[inspected[0]][inspected[1]] = Square::Empty;
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

impl Game {
    pub fn new() -> Game {
        let mut new_game = Game {
            state: GameState::Running,
            board: Board::new(),
        };
        new_game.restart();
        new_game
    }

    pub fn execute(&mut self, command: &Command) {
        match command {
            Command::Nop => (), // screen refresh only
            Command::Left => self.shift_left(),
            Command::Right => self.shift_right(),
            Command::Up => self.shift_up(),
            Command::Down => self.shift_down(),
            Command::New => self.restart(),
            Command::Quit => self.state = GameState::Finished,
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    fn restart(&mut self) {
        self.board.initialize();
    }

    fn shift_left(&mut self) {
        self.board.shift_left();
        self.board.new_tile();
    }

    fn shift_right(&mut self) {
        self.board.shift_right();
        self.board.new_tile();
    }

    fn shift_up(&mut self) {
        self.board.shift_up();
        self.board.new_tile();
    }

    fn shift_down(&mut self) {
        self.board.shift_down();
        self.board.new_tile();
    }
}

struct Stepper {
    position: [usize; 2],
    step: [i8; 2],
    end: [usize; 2],
}

impl Stepper {
    fn new(start: [usize; 2], step: [i8; 2], end: [usize; 2]) -> Stepper {
        Stepper {
            position: start,
            step,
            end,
        }
    }
    fn finished(&self) -> bool {
        self.position == self.end
    }
    fn position(&self) -> [usize; 2] {
        self.position
    }
    fn advance(&mut self) {
        let mut pos = self.position;
        self.advance_position(&mut pos); // TODO: learn how to NOT use a temp var here
        self.position = pos;
    }
    fn advance_position(&self, position: &mut [usize; 2]) {
        *position = [
            (position[0] as i8 + self.step[0]) as usize,
            (position[1] as i8 + self.step[1]) as usize,
        ];
    }
}
