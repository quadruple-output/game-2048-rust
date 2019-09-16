mod console;
mod ncurses;

use crate::game::Command;

pub use self::ncurses::NCursesController;
pub use console::ConsoleController;

pub trait Controller {
    fn receive_command(&self) -> Command;
}
