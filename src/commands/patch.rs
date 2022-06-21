use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::*;

use crate::lib::config;
use crate::lib::output::*;

/// Convert bytes to gigabytes with 2 digits round
fn format_size(bytes: u64) -> f64 {
    (bytes as f64 / 1024.0 / 1024.0 / 1024.0 * 100.0).ceil() / 100.0
}

pub struct Patch {
    args: Vec<Box<dyn Argument>>
}

impl Patch {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--game", vec!["--path", "-g", "-p"], true)
            ]
        })
    }
}

impl Command for Patch {
    fn get_name(&self) -> &str {
        "patch"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, values: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        let game = if values.len() > 0 {
            Game::new(values[0].value.clone())
        } else {
            if config.paths.game == "" {
                notice("You didn't specify the game path\n");
            }

            Game::new(config.paths.game)
        };

        // todo

        true
    }
}
