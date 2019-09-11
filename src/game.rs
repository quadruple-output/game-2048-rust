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
    pub grid: [[Square; 4]; 4],
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
        Board {
            size: 4, // TODO: to make this a variable, the type of 'grid' needs to be non-array
            grid: [[Square::Empty; 4]; 4],
            rand_range_grid: rand::distributions::Range::new(0, 4),
            rand_range_10: rand::distributions::Range::new(0, 10),
            rng: rand::thread_rng(),
        }
    }

    fn initialize(&mut self) {
        self.grid = [[Square::Empty; 4]; 4];
        self.new_tile();
    }

    fn new_tile(&mut self) {
        let x = self.random_grid_size();
        let y = self.random_grid_size();
        self.grid[x][y] = Square::Value(if self.ten_percent_chance() { 4 } else { 2 });
    }

    fn ten_percent_chance(&mut self) -> bool {
        self.rand_range_10.ind_sample(&mut self.rng) == 0
    }

    fn random_grid_size(&mut self) -> usize {
        self.rand_range_grid.ind_sample(&mut self.rng)
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
            Command::Nop => (),
            Command::Right | Command::Left | Command::Up | Command::Down => {
                self.board.new_tile();
            }
            Command::New => self.restart(),
            Command::Quit => {
                self.state = GameState::Finished;
            }
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    fn restart(&mut self) {
        self.board.initialize();
    }
}
