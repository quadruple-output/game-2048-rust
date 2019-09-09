mod console; // not public
mod ncurses;

pub use console::Controller as ConsoleController;
pub use ncurses::Controller as NCursesController;

enum Command {
    Nop,
    Right,
    Left,
    Up,
    Down,
    New,
    Quit,
}
