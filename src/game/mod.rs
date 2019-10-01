mod board;

pub use board::{Board, Move, Square};

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
    Quit,
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

    pub fn execute(&mut self, command: Command) -> Option<Vec<Move>> {
        match command {
            Command::Nop => None, // screen refresh only
            Command::Left => self.shift_left(),
            Command::Right => self.shift_right(),
            Command::Up => self.shift_up(),
            Command::Down => self.shift_down(),
            Command::New => {
                self.restart();
                None
            }
            Command::Quit => {
                self.state = GameState::Quit;
                None
            }
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    fn restart(&mut self) {
        self.board.initialize();
    }

    fn shift_left(&mut self) -> Option<Vec<Move>> {
        match self.board.shift_left() {
            Ok(moves) => {
                self.board.new_tile();
                Some(moves)
            }
            Err(_) => None,
        }
    }

    fn shift_right(&mut self) -> Option<Vec<Move>> {
        match self.board.shift_right() {
            Ok(moves) => {
                self.board.new_tile();
                Some(moves)
            }
            Err(_) => None,
        }
    }

    fn shift_up(&mut self) -> Option<Vec<Move>> {
        match self.board.shift_up() {
            Ok(moves) => {
                self.board.new_tile();
                Some(moves)
            }
            Err(_) => None,
        }
    }

    fn shift_down(&mut self) -> Option<Vec<Move>> {
        match self.board.shift_down() {
            Ok(moves) => {
                self.board.new_tile();
                Some(moves)
            }
            Err(_) => None,
        }
    }
}
