mod board;

pub use board::{Board, Square};

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

pub struct Game {
    pub board: Board,
    state: GameState,
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

    pub fn execute(&mut self, command: &Command) -> Result<(), ()> {
        match command {
            Command::Nop => Ok(()), // screen refresh only
            Command::Left => self.shift_left(),
            Command::Right => self.shift_right(),
            Command::Up => self.shift_up(),
            Command::Down => self.shift_down(),
            Command::New => {
                self.restart();
                Ok(())
            }
            Command::Quit => {
                self.state = GameState::Finished;
                Ok(())
            }
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    fn restart(&mut self) {
        self.board.initialize();
    }

    fn shift_left(&mut self) -> Result<(), ()> {
        self.board.shift_left();
        self.board.new_tile()
    }

    fn shift_right(&mut self) -> Result<(), ()> {
        self.board.shift_right();
        self.board.new_tile()
    }

    fn shift_up(&mut self) -> Result<(), ()> {
        self.board.shift_up();
        self.board.new_tile()
    }

    fn shift_down(&mut self) -> Result<(), ()> {
        self.board.shift_down();
        self.board.new_tile()
    }
}
