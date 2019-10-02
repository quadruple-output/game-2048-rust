use std::env;

use game2048::{run, ViewType};

pub struct Config {
	prog_name: String,
	show_version: bool,
	show_usage: bool,
	view_type: ViewType,
}

impl Config {
	pub fn from_args(args: Vec<String>) -> Self {
		let mut show_usage = false;
		let mut show_version = false;
		let mut view_type = ViewType::NCurses;
		if args.len() > 2 {
			show_usage = true;
		} else if args.len() == 2 {
			match args[1].as_str() {
				"-v" => show_version = true,
				"-c" => view_type = ViewType::Console,
				_ => show_usage = true,
			}
		}
		Config { prog_name: args[0].clone(), show_version, show_usage, view_type }
	}
}

fn main() {
	let config = Config::from_args(env::args().collect());
	if config.show_usage {
		println!("Usage: {} [-v|-c]", config.prog_name);
		println!("  -v : show version");
		println!("  -c : use plain console output instead of ncurses library")
	}
	if config.show_version {
		println!("Game2048 by Ivo.  V 0.1.0");
	}
	if !config.show_usage && !config.show_version {
		run(config.view_type);
	}
}
