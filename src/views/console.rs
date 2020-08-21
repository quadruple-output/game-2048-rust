use ::console::Term;
use ::std::cell::RefCell;

use super::View;
use crate::game::{Board, Game, Square};

pub struct ConsoleView<'a> {
  game: &'a RefCell<Game>,
  term: Term
}

impl<'a> View for ConsoleView<'a> {
  fn update(&self) { self.show_board(&self.game.borrow().board); }
}


impl<'a> ConsoleView<'a> {
  pub fn new(game: &RefCell<Game>) -> ConsoleView {
    let term = Term::stdout();
    term.set_title("2048");
    ConsoleView { game, term }
  }

  pub fn term(&self) -> &Term { &self.term }

  fn show_board(&self, board: &Board) {
    println!();
    for y in 0..board.size_y() {
      for x in 0..board.size_x() {
        match board.at_xy(x, y) {
          Square::Empty => print!("[     ]"),
          Square::Value(v) => print!("[{0:^5}]", v)
        }
      }
      println!();
    }
  }
}

impl<'a> Drop for ConsoleView<'a> {
  fn drop(&mut self) { self.term.set_title(""); }
}
