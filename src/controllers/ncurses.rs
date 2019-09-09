use crate::game::Command;

#[allow(dead_code)]
pub struct Controller {}

impl Controller {
    #[allow(dead_code)]
    pub fn new() -> Controller {
        Controller {}
    }

    #[allow(dead_code)]
    pub fn receive_command(&self) -> Command {
        Command::Quit
    }
}
