use std::rc::Rc;
use std::cell::RefCell;
use ncurses as nc;

use super::Controller;
use crate::game::{Game, Command};

#[allow(dead_code)]
pub struct NCursesController {
    game: Rc<RefCell<Game>>,
}

impl Controller for NCursesController {
    fn receive_command(&self) -> Command {
        loop {
            let key = nc::getch();
            match key {
                nc::KEY_RESIZE => break Command::Nop, // window resize event
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

impl NCursesController {
    #[allow(dead_code)]
    pub fn new(game: Rc<RefCell<Game>>) -> impl Controller {
        nc::cbreak();
        nc::keypad(nc::stdscr(), true);
        nc::noecho();
        NCursesController {game}
    }
}
