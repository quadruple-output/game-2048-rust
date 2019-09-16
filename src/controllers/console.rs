use super::Controller;
use crate::game::Command;
use std::io;

#[allow(dead_code)]
pub struct ConsoleController {}

impl Controller for ConsoleController {
    fn receive_command(&self) -> Command {
        let mut cmd;
        loop {
            cmd = String::new();
            match io::stdin().read_line(&mut cmd) {
                Ok(_) => match cmd.to_lowercase().as_str().trim() {
                    "w" => break Command::Up,
                    "a" => break Command::Left,
                    "s" => break Command::Down,
                    "d" => break Command::Right,
                    "n" => break Command::New,
                    "q" => break Command::Quit,
                    _ => println!("what?"), // restarts the loop
                },
                Err(msg) => {
                    println!("I/O Error on STDIN: {}", msg);
                    break Command::Quit;
                }
            }
        }
    }
}

impl ConsoleController {
    #[allow(dead_code)]
    pub fn new() -> impl Controller {
        ConsoleController {}
    }
}
