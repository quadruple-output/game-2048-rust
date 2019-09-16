mod console;
mod ncurses;

pub use self::ncurses::NCursesView;
pub use self::console::ConsoleView;

use crate::game::Game;

pub trait View {
    fn show(&self, game: &Game);
}
