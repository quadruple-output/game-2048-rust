use std::cell::{Ref, RefMut};

use crate::game::{Command, Game, GameState};
use crate::views::View;

pub use self::ncurses::NCursesController;
pub use console::ConsoleController;

mod console;
mod ncurses;

pub trait Controller {
    fn receive_command(&self) -> Command;

    fn view(&self) -> &dyn View;

    fn game(&self) -> Ref<Game>;

    fn mut_game(&self) -> RefMut<Game>;

    fn run_game(&self) {
        loop {
            self.view().update();
            self.mut_game().execute(self.receive_command());
            if let GameState::Quit = self.game().state() {
                break;
            }
        }
    }
}
