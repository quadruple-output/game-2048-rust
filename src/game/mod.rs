mod board;

pub use board::{Board, Coord, Move, Square};

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
	Over, // Game Over - cannot continue playing
	Quit  // Game ended by user request
}

pub struct Game {
	pub board:    Board,
	state:        GameState,
	latest_moves: Vec<Move>
}

impl Game {
	pub fn new(size_x: usize, size_y: usize) -> Game {
		let mut new_game = Game { state:        GameState::Running,
		                          board:        Board::new(size_x, size_y),
		                          latest_moves: Vec::new() };
		new_game.execute(Command::New);
		new_game
	}

	pub fn execute(&mut self, command: Command) {
		if let Some(new_moves) = match command {
			Command::Nop => None, // screen refresh only
			Command::Left => self.shift_left(),
			Command::Right => self.shift_right(),
			Command::Up => self.shift_up(),
			Command::Down => self.shift_down(),
			Command::New => Some(vec![self.restart()]),
			Command::Quit => {
				self.state = GameState::Quit;
				None
			}
		} {
			self.latest_moves = new_moves;
		}
	}

	pub fn state(&self) -> &GameState { &self.state }

	pub fn latest_moves(&self) -> &Vec<Move> { &self.latest_moves }

	fn restart(&mut self) -> Move { self.board.initialize() }

	fn shift_left(&mut self) -> Option<Vec<Move>> {
		match self.board.shift_left() {
			Ok(mut moves) => {
				moves.push(self.board.new_tile());
				Some(moves)
			},
			Err(_) => None
		}
	}

	fn shift_right(&mut self) -> Option<Vec<Move>> {
		match self.board.shift_right() {
			Ok(mut moves) => {
				moves.push(self.board.new_tile());
				Some(moves)
			},
			Err(_) => None
		}
	}

	fn shift_up(&mut self) -> Option<Vec<Move>> {
		match self.board.shift_up() {
			Ok(mut moves) => {
				moves.push(self.board.new_tile());
				Some(moves)
			},
			Err(_) => None
		}
	}

	fn shift_down(&mut self) -> Option<Vec<Move>> {
		match self.board.shift_down() {
			Ok(mut moves) => {
				moves.push(self.board.new_tile());
				Some(moves)
			},
			Err(_) => None
		}
	}
}
