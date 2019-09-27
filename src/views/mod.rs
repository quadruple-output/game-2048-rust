mod console;
mod ncurses;

pub use self::console::ConsoleView;
pub use self::ncurses::NCursesView;

pub trait View {
    fn update(&self);
}
