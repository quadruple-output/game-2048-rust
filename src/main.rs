use std::io;
mod game;
use game::Board;

fn main() {
    let mut board = Board::new();
    let view = ConsoleView::new();
    let controller = ConsoleController::new();

    board.new_tile();
    loop {
        view.show_board(&board);
        match controller.effectuate(&mut board) {
            GameState::Finished => break,
            GameState::Running => (),
        }
    }
}

pub struct ConsoleView {}

pub struct ConsoleController {}

impl ConsoleView {
    pub fn new() -> ConsoleView {
        ConsoleView {}
    }

    pub fn show_board(&self, board: &Board) {
        for x in 0..4 {
            for y in 0..4 {
                print!(" {:?} ", board.grid[x][y]);
            }
            println!("");
        }
    }
}

impl ConsoleController {
    pub fn new() -> ConsoleController {
        ConsoleController {}
    }

    pub fn effectuate(&self, board: &mut Board) -> GameState {
        match self.read_command() {
            Command::Right | Command::Left | Command::Up | Command::Down => {
                board.new_tile();
            }
            Command::New => {
                board.restart();
                board.new_tile();
            }
            Command::Quit => return GameState::Finished,
            Command::Nop => (),
        }
        GameState::Running
    }

    fn read_command(&self) -> Command {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        match cmd.to_lowercase().as_str().trim() {
            "w" => Command::Up,
            "a" => Command::Left,
            "s" => Command::Down,
            "d" => Command::Right,
            "n" => Command::New,
            "q" => Command::Quit,
            _ => Command::Nop,
        }
    }
}

pub enum GameState {
    Running,
    Finished,
}

enum Command {
    Nop,
    Right,
    Left,
    Up,
    Down,
    New,
    Quit,
}
