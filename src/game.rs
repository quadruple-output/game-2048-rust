use rand::Rng;

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
    pub grid: [[Square; 4]; 4],
}

#[derive(Copy, Clone)] // needed for easy Board initialization
#[derive(Debug)] // only needed for console view. TODO: remove or define in views.rs
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

    pub fn initialize(&mut self) {
        self.grid = [[Square::Empty; 4]; 4];
        self.new_tile();
    }

    pub fn new_tile(&mut self) {
        let x = rand::thread_rng().gen_range(0, 4);
        let y = rand::thread_rng().gen_range(0, 4);
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
