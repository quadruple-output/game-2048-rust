use std::cmp::Ordering::*;
use std::env;

use game2048::{ViewType, ViewType::*};

enum Config {
	Play { view_type: ViewType },
	ShowVersion,
	ShowUsage { cmd_name: String }
}

impl Config {
	pub fn from_args(args: Vec<String>) -> Self {
		match args.len().cmp(&2) {
			Less => Self::Play { view_type: NCurses },
			Equal => match args[1].as_str() {
				"-v" => Self::ShowVersion,
				"-c" => Self::Play { view_type: Console },
				_ => Self::ShowUsage { cmd_name: args[0].clone() }
			},
			Greater => Self::ShowUsage { cmd_name: args[0].clone() }
		}
	}
}

fn main() {
	match Config::from_args(env::args().collect()) {
		Config::Play { view_type } => game2048::play(view_type, 3, 3),
		Config::ShowVersion => println!("Game2048 by Ivo.  V 0.1.0"),
		Config::ShowUsage { cmd_name } => {
			println!("Usage: {} [-v|-c]", cmd_name);
			println!("  -v : show version");
			println!("  -c : use plain console output instead of ncurses library")
		}
	}
}
