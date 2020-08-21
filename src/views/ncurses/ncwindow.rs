use log::trace;
use ncurses as nc;
use std::fmt;

// "Newtype" wrapper pattern for implementing Drop for nc::WINDOW
pub struct NCWindow(pub nc::WINDOW, String /* just a label for debugging – no functionality */);


impl NCWindow {
  pub fn new(parent: Option<&NCWindow>, wrappee: nc::WINDOW, label: &str) -> Self {
    let mut new_label = String::new();
    if let Some(parent) = parent {
      new_label.push_str(&parent.1);
      new_label.push_str("::");
    };
    new_label.push_str(label);
    let new_win = NCWindow(wrappee, new_label);
    trace!("new Window {:?}", new_win);
    new_win
  }

  pub fn size(&self) -> (i32, i32) {
    let (mut win_height, mut win_width) = (0, 0);
    nc::getmaxyx(self.0, &mut win_height, &mut win_width);
    (win_height, win_width)
  }
}


impl fmt::Debug for NCWindow {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (height, width) = self.size();
    let (mut x, mut y) = (0, 0);
    nc::getparyx(self.0, &mut y, &mut x);
    write!(f, "{:?}: '{}' {}x{}@{},{}", self.0, self.1, width, height, x, y)
    // f.debug_struct("NCWindow").field("x", &self.x).field("y",
    // &self.y).finish()
  }
}


impl Drop for NCWindow {
  fn drop(&mut self) {
    // delwin is important to clear up ncurses. Otherwise, a screen redraw on window
    // resize takes more and more time the longer you play.
    trace!("drop Window: {:?}", self.0);
    assert!(nc::ERR != nc::delwin(self.0));
  }
}
