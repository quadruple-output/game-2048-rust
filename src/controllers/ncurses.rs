use super::Command;
use crate::game::*;
use std::io;

#[allow(dead_code)]
pub struct Controller {}

impl Controller {
    #[allow(dead_code)]
    pub fn new() -> Controller {
        Controller {}
    }

    #[allow(dead_code)]
    pub fn initialize_game(&self, board: &mut Board) {
        board.new_tile(); // TODO: should be inherited
    }

    #[allow(dead_code)]
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
