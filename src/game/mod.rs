mod board;

pub use board::{Board, Coord, Move, Randomizer, Square};
use log::info;

#[derive(Debug)]
pub enum Command {
  Nop, // no operation. Used for repainting
  New,
  Quit,
  Right,
  Left,
  Up,
  Down
}

pub enum GameState {
  Running,
  // Over, // Game Over - cannot continue playing
  Quit // Game ended by user request
}

pub struct Game {
  pub board:    Board,
  randomizer:   Randomizer,
  state:        GameState,
  latest_moves: Vec<Move>,
  move_count:   usize
}

impl Game {
  pub fn new(size_x: usize, size_y: usize) -> Game {
    let mut new_game = Game { state:        GameState::Running,
                              board:        Board::new(size_x, size_y),
                              randomizer:   Randomizer::new(),
                              latest_moves: Vec::new(),
                              move_count:   0 };
    new_game.execute(Command::New);
    new_game
  }

  pub fn execute(&mut self, command: Command) {
    match match command {
            Command::Nop => None, // screen refresh only
            Command::Left => self.shift_left(),
            Command::Right => self.shift_right(),
            Command::Up => self.shift_up(),
            Command::Down => self.shift_down(),
            Command::New => Some(vec![self.restart()]),
            Command::Quit => {
              self.state = GameState::Quit;
              info!("Game command: {:?}", command);
              return;
            }
          } {
      Some(new_moves) => {
        info!("Game command: {:?}", command);
        self.latest_moves = new_moves;
        self.move_count += 1;
      },
      None => info!("Game command: {:?} (no move)", command)
    }
  }

  pub fn state(&self) -> &GameState { &self.state }

  pub fn latest_moves(&self) -> &[Move] { &self.latest_moves }

  pub fn move_count(&self) -> usize { self.move_count }

  fn restart(&mut self) -> Move { self.board.initialize(&mut self.randomizer) }

  fn shift_left(&mut self) -> Option<Vec<Move>> {
    match self.board.shift_left() {
      Some(mut moves) => {
        moves.push(self.board.new_tile(&mut self.randomizer));
        Some(moves)
      },
      None => None
    }
  }

  fn shift_right(&mut self) -> Option<Vec<Move>> {
    match self.board.shift_right() {
      Some(mut moves) => {
        moves.push(self.board.new_tile(&mut self.randomizer));
        Some(moves)
      },
      None => None
    }
  }

  fn shift_up(&mut self) -> Option<Vec<Move>> {
    match self.board.shift_up() {
      Some(mut moves) => {
        moves.push(self.board.new_tile(&mut self.randomizer));
        Some(moves)
      },
      None => None
    }
  }

  fn shift_down(&mut self) -> Option<Vec<Move>> {
    match self.board.shift_down() {
      Some(mut moves) => {
        moves.push(self.board.new_tile(&mut self.randomizer));
        Some(moves)
      },
      None => None
    }
  }
}
