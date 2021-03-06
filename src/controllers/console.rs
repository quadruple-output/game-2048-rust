use console::Key;
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
    loop {
      match self.view.term().read_key() {
        Ok(key) => match key {
          Key::Unknown => {
            // terminal appears not to be "user attended". Unfortunately, this is the case
            // for the Eclipse console.
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
              Ok(_) => {
                match line.to_lowercase().as_str().trim() {
                  "w" => break Command::Up,
                  "a" => break Command::Left,
                  "s" => break Command::Down,
                  "d" => break Command::Right,
                  "n" => break Command::New,
                  "q" => break Command::Quit,
                  _ => println!("try W, A, S, D, N(ew), or Q(uit)") // restarts the loop
                }
              },
              Err(msg) => {
                println!("I/O Error on STDIN: {}", msg);
                break Command::Quit;
              }
            }
          },
          Key::ArrowUp => break Command::Up,
          Key::ArrowLeft => break Command::Left,
          Key::ArrowDown => break Command::Down,
          Key::ArrowRight => break Command::Right,
          Key::Char('n') => break Command::New,
          Key::Char('q') => break Command::Quit,
          _ => println!("try arrow keys, N(ew), or Q(uit)") // restarts the loop
        },
        Err(msg) => {
          println!("I/O Error on STDIN: {}", msg);
          break Command::Quit;
        }
      }
    }
  }
}
