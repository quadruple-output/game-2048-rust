use clap::{App, Arg}; // argument parser ⇒ https://docs.rs/clap/
                      // use stderrlog; // tracing ⇒ https://docs.rs/log + https://docs.rs/stderrlog

use game2048::ViewType;

const AUTHOR: &str = "quadruple-output";
const VERSION: &str = "0.1.1";

const ARG_VERBOSITY: &str = "verbosity";
const ARG_VIEW_TYPE: &str = "view_type";
const ARG_LIST_VIEW_TYPES: &str = "list_types";
const ARG_BOARD_WIDTH: &str = "width";
const ARG_BOARD_HEIGHT: &str = "height";

fn main() {
  let args = App::new("2048 (Example in Rust)")
    .author(AUTHOR)
    .version(VERSION)
    .about("This is a board game, conceptually based on http://git.io/2048 ")
    .arg(
      Arg::with_name(ARG_VIEW_TYPE)
        .short("d")
        .value_name("DISPLAY_TYPE")
        .help("Sets the display type. See -l."),
    )
    .arg(Arg::with_name(ARG_LIST_VIEW_TYPES).short("l").help("Lists available display types and exits"))
    .arg(
      Arg::with_name(ARG_VERBOSITY)
        .short("v")
        .multiple(true)
        .help("Sets level of verbosity on stderr. Repeat up to 4 times for more details"),
    )
    .arg(
      Arg::with_name(ARG_BOARD_WIDTH)
        .short("w")
        .value_name("WIDTH")
        .default_value("3")
        .help("Set the width of the Board"),
    )
    .arg(
      Arg::with_name(ARG_BOARD_HEIGHT)
        .short("h")
        .value_name("HEIGHT")
        .default_value("3")
        .help("Set the height of the Board"),
    )
    .get_matches();

  if args.is_present(ARG_LIST_VIEW_TYPES) {
    println!("ncurses");
    println!("console");
  } else {
    stderrlog::new()
      .module(module_path!())
      .verbosity(args.occurrences_of(ARG_VERBOSITY) as usize)
      .color(stderrlog::ColorChoice::Always)
      .init()
      .unwrap();
    if let Some(view_type) = match args.value_of(ARG_VIEW_TYPE) {
      None | Some("console") => Some(ViewType::Console),
      Some("ncurses") => Some(ViewType::NCurses),
      Some(other) => {
        println!("Unknown display type \"{}\". Try -l.", other);
        None
      },
    } {
      let width = args.value_of(ARG_BOARD_WIDTH).unwrap().parse().unwrap_or(3);
      let height = args.value_of(ARG_BOARD_HEIGHT).unwrap().parse().unwrap_or(3);
      game2048::play(view_type, width, height);
    }
  }
}
