use ncurses as nc;

pub struct Pallete {}

impl Pallete {
  pub fn new() -> Self {
    // backgrounds:
    nc::init_color(100, 1000, 1000, 1000); // white
    nc::init_color(101, 500, 1000, 500); // light green
    nc::init_color(102, 900, 900, 333); // light yellow
    nc::init_color(103, 1000, 500, 500); // light red
    nc::init_color(104, 900, 333, 900); // light magenta
    nc::init_color(105, 500, 500, 1000); // light blue
    nc::init_color(106, 333, 900, 900); // light cyan
    nc::init_color(107, 0, 1000, 0); // green
    nc::init_color(108, 900, 900, 0); // yellow
    nc::init_color(109, 1000, 0, 0); // red
    nc::init_color(110, 900, 0, 900); // magenta
    nc::init_color(111, 0, 0, 1000); // blue
    nc::init_color(112, 0, 900, 900); // cyan

    // foregrounds:
    nc::init_color(200, 1000, 0, 0); // red
    nc::init_color(201, 0, 0, 0); // black
    nc::init_color(202, 0, 0, 0); // black
    nc::init_color(203, 0, 0, 0); // black
    nc::init_color(204, 0, 0, 0); // black
    nc::init_color(205, 0, 0, 0); // black
    nc::init_color(206, 0, 0, 0); // black
    nc::init_color(207, 0, 0, 0); // black
    nc::init_color(208, 0, 0, 0); // black
    nc::init_color(209, 0, 0, 0); // black
    nc::init_color(210, 0, 0, 0); // black
    nc::init_color(211, 0, 0, 0); // black
    nc::init_color(212, 0, 0, 0); // black

    // combinations:
    nc::init_pair(1, 201, 101);
    nc::init_pair(2, 202, 102);
    nc::init_pair(3, 203, 103);
    nc::init_pair(4, 204, 104);
    nc::init_pair(5, 205, 105);
    nc::init_pair(6, 206, 106);
    nc::init_pair(7, 207, 107);
    nc::init_pair(8, 208, 108);
    nc::init_pair(9, 209, 109);
    nc::init_pair(10, 210, 110);
    nc::init_pair(11, 211, 111);
    nc::init_pair(12, 212, 112);
    nc::init_pair(13, 200, 100);
    Self {}
  }

  pub fn get_pair_for_square_value(&self, value: u16) -> nc::attr_t {
    match value {
      0 => nc::COLOR_PAIR(13), // special flash-color for newly appearing tile
      2 => nc::COLOR_PAIR(1),
      4 => nc::COLOR_PAIR(2),
      8 => nc::COLOR_PAIR(3),
      16 => nc::COLOR_PAIR(4),
      32 => nc::COLOR_PAIR(5),
      64 => nc::COLOR_PAIR(6),
      128 => nc::COLOR_PAIR(7),
      256 => nc::COLOR_PAIR(8),
      512 => nc::COLOR_PAIR(9),
      1024 => nc::COLOR_PAIR(10),
      2048 => nc::COLOR_PAIR(11),
      _ => nc::COLOR_PAIR(12)
    }
  }
}
