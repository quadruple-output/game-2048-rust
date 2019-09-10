mod console;
mod ncurses;

pub use console::Controller as ConsoleController;
pub use self::ncurses::Controller as NCursesController;
