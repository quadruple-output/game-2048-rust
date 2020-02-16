use clap; // todo: use better argument parser for tracing ⇒ https://docs.rs/clap/
use std::cmp::Ordering::*;
use std::env;
use stderrlog;

use game2048::{ViewType, ViewType::*};

const AUTHOR: &str = "quadruple-output";
const VERSION: &str = "0.1.1";

enum Config {
	Play { view_type: ViewType },
	ShowVersion,
	ShowUsage { cmd_name: String }
}

impl Config {
	pub fn from_args(args: Vec<String>) -> Self {
		match args.len().cmp(&2) {
			Less => Config::Play { view_type: NCurses },
			Equal => match args[1].as_str() {
				"-v" => Config::ShowVersion,
				"-c" => Config::Play { view_type: Console },
				_ => Config::ShowUsage { cmd_name: args[0].clone() }
			},
			Greater => Config::ShowUsage { cmd_name: args[0].clone() }
		}
	}
}

fn main() {
	// stderrlog::new().module(module_path!()).verbosity(5).init().unwrap();
	match Config::from_args(env::args().collect()) {
		Config::Play { view_type } => game2048::play(view_type, 3, 3),
		Config::ShowVersion => println!("Game2048 by {}.  V {}", AUTHOR, VERSION),
		Config::ShowUsage { cmd_name } => {
			println!("Usage: {} [-v|-c]", cmd_name);
			println!("  -v : show version");
			println!("  -c : use plain console output instead of ncurses library")
		}
	}
}
