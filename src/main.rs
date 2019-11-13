use std::cmp::Ordering::*;
use std::env;

use game2048::{run, ViewType, ViewType::*};

enum Config {
	Play { view_type: ViewType },
	ShowVersion,
	ShowUsage { cmd_name: String }
}

impl Config {
	pub fn from_args(args: Vec<String>) -> Self {
		match args.len().cmp(&2) {
			Less => return Self::Play { view_type: NCurses },
			Equal => match args[1].as_str() {
				"-v" => return Self::ShowVersion,
				"-c" => return Self::Play { view_type: Console },
				_ => {}
			},
			Greater => {}
		};
		Self::ShowUsage { cmd_name: args[0].clone() }
	}
}

fn main() {
	match Config::from_args(env::args().collect()) {
		Config::Play { view_type } => run(view_type),
		Config::ShowVersion => println!("Game2048 by Ivo.  V 0.1.0"),
		Config::ShowUsage { cmd_name } => {
			println!("Usage: {} [-v|-c]", cmd_name);
			println!("  -v : show version");
			println!("  -c : use plain console output instead of ncurses library")
		}
	}
}
