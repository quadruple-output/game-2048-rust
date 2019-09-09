mod console; // not public

pub use console::Controller as ConsoleController;

enum Command {
    Nop,
    Right,
    Left,
    Up,
    Down,
    New,
    Quit,
}
