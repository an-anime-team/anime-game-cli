use std::process::Command as ProcessCommand;

use commandor::prelude::*;

use crate::lib::config;
use crate::lib::output::*;

pub struct Run {
    args: Vec<Box<dyn Argument>>
}

impl Run {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![]
        })
    }
}

impl Command for Run {
    fn get_name(&self) -> &str {
        "run"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, _: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        let child = ProcessCommand::new(config.wine.executable)
            .envs(config.wine.environment)
            .env("WINEPREFIX", &config.wine.prefix)
            .current_dir(config.paths.game)
            .arg("launcher.bat")
            .spawn();

        match child {
            Ok(_) => (),
            Err(err) => error(format!("Game running error: {}", err))
        }

        true
    }
}
