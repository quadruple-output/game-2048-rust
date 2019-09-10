use crate::game::Command;
use ncurses as nc;

#[allow(dead_code)]
pub struct Controller {}

impl Controller {
    #[allow(dead_code)]
    pub fn new() -> Controller {
        nc::cbreak();
        nc::keypad(nc::stdscr(), true);
        nc::noecho();
        Controller {}
    }

    #[allow(dead_code)]
    pub fn receive_command(&self) -> Command {
        loop {
            let key = nc::getch();
            match key {
                nc::KEY_LEFT => break Command::Left,
                nc::KEY_RIGHT => break Command::Right,
                nc::KEY_UP => break Command::Up,
                nc::KEY_DOWN => break Command::Down,
                _ => (),
            }
            match key as u8 as char {
                'n' | 'N' => break Command::New,
                'q' | 'Q' => break Command::Quit,
                _ => (),
            }
        }
    }
}
