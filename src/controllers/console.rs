use std::cell::{Ref, RefCell, RefMut};
use std::io;

use super::Controller;
use crate::game::{Command, Game};
use crate::views::{ConsoleView, View};

pub struct ConsoleController<'a> {
  game: &'a RefCell<Game>,
  view: ConsoleView<'a>
}

impl<'a> ConsoleController<'a> {
  pub fn create(game: &'a RefCell<Game>, view: ConsoleView<'a>) -> ConsoleController<'a> {
    ConsoleController { game, view }
  }
}

impl<'a> Controller for ConsoleController<'a> {
  fn view(&self) -> &dyn View { &self.view }

  fn game(&self) -> Ref<Game> { self.game.borrow() }

  fn mut_game(&self) -> RefMut<Game> { self.game.borrow_mut() }

  fn receive_command(&self) -> Command {
    let mut cmd;
    loop {
      cmd = String::new();
      match io::stdin().read_line(&mut cmd) {
        Ok(_) => match cmd.to_lowercase().as_str().trim() {
          "w" => break Command::Up,
          "a" => break Command::Left,
          "s" => break Command::Down,
          "d" => break Command::Right,
          "n" => break Command::New,
          "q" => break Command::Quit,
          _ => println!("what?") // restarts the loop
        },
        Err(msg) => {
          println!("I/O Error on STDIN: {}", msg);
          break Command::Quit;
        }
      }
    }
  }
}
