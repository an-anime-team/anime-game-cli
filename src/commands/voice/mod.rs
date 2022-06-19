use commandor::prelude::*;
use colorful::Colorful;

use animegame_core::game::Game;

pub struct Voice {
    args: Vec<Box<dyn Argument>>
}

impl Voice {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--game", vec!["--path", "-g", "-p"], false)
            ]
        })
    }
}

impl Command for Voice {
    fn get_name(&self) -> &str {
        "voice"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, values: Vec<ArgumentValue>) -> bool {
        // TODO: voice info, voice add, voice remove

        true
    }
}
