use clap::{App, Arg}; // argument parser ⇒ https://docs.rs/clap/
use stderrlog;

use game2048::ViewType;

const AUTHOR: &str = "quadruple-output";
const VERSION: &str = "0.1.1";


fn main() {
	let args =
		App::new("2048 (Example in Rust)").author(AUTHOR)
		                                  .version(VERSION)
		                                  .about("This is a board game, conceptually based on \
		                                          http://git.io/2048 ")
		                                  .arg(Arg::with_name("view").short("d")
		                                                             .value_name("DISPLAY_TYPE")
		                                                             .help("Sets the display type. See -l. \
		                                                                    Default is \"ncurses\"."))
		                                  .arg(Arg::with_name("list_views").short("l")
		                                                                   .help("Lists available display \
		                                                                          types and exits"))
		                                  .arg(Arg::with_name("verbosity").short("v")
		                                                                  .multiple(true)
		                                                                  .help("Sets level of verbosity on \
		                                                                         stderr"))
		                                  .get_matches();

	if args.is_present("list_views") {
		println!("ncurses");
		println!("console");
	} else {
		// stderrlog::new().module(module_path!()).verbosity(5).init().unwrap();
		if let Some(view_type) = match args.value_of("view") {
			None | Some("ncurses") => Some(ViewType::NCurses),
			Some("console") => Some(ViewType::Console),
			Some(other) => {
				println!("Unknown display type \"{}\". Try -l.", other);
				None
			}
		} {
			game2048::play(view_type, 3, 3);
		}
	}
}
