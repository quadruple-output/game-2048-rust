use crate::game::Command;
use std::io;

#[allow(dead_code)]
pub struct Controller {}

impl Controller {
    #[allow(dead_code)]
    pub fn new() -> Controller {
        Controller {}
    }

    #[allow(dead_code)]
    pub fn receive_command(&self) -> Command {
        let mut cmd;
        loop {
            cmd = String::new();
            io::stdin().read_line(&mut cmd).unwrap();
            let some_command = match cmd.to_lowercase().as_str().trim() {
                "w" => Some(Command::Up),
                "a" => Some(Command::Left),
                "s" => Some(Command::Down),
                "d" => Some(Command::Right),
                "n" => Some(Command::New),
                "q" => Some(Command::Quit),
                _ => None,
            };
            if let Some(command) = some_command {
                break command;
            } else {
                println!("what?");
            }
        }
    }
}
