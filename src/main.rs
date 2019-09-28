use std::env;

use game2048::{run, ViewType};

pub struct Config {
    show_version: bool,
    view_type: ViewType,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> Self {
        Config {
            show_version: false,
            view_type: ViewType::NCurses,
        }
    }
}

fn main() {
    let config = Config::from_args(env::args().collect());
    run(config.view_type);
}
