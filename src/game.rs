use rand::distributions::{IndependentSample, Range};

pub struct Game {
    pub board: Board,
    state: GameState,
}

pub enum Command {
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
    pub size: u8,
    pub grid: [[Square; 4]; 4],
    rand_range: Range<usize>, // array indexes must be typed 'usize'
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
            rand_range: Range::new(0, 4),
            rng: rand::thread_rng(),
        }
    }

    fn initialize(&mut self) {
        self.grid = [[Square::Empty; 4]; 4];
        self.new_tile();
    }

    fn new_tile(&mut self) {
        let x = self.rand_range.ind_sample(&mut self.rng);
        let y = self.rand_range.ind_sample(&mut self.rng);
        self.grid[x][y] = Square::Value(2);
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
