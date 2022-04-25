use commandor::prelude::*;
use animegame_core::game::Game;

pub struct Info {
    args: Vec<Box<dyn Argument>>
}

impl Info {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--game", vec!["--path", "-g", "-p"], false)
            ]
        })
    }
}

impl Command for Info {
    fn get_name(&self) -> &str {
        "info"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, values: Vec<ArgumentValue>) -> bool {
        let mut game = Game::new(values[0].value.clone());

        let game_version = game.version();

        println!(" Installed version: {}", {
            match game_version.installed() {
                Ok(version) => version.to_string(),
                Err(_) => "?".to_string()
            }
        });

        println!(" Latest version: {}", {
            match game_version.latest() {
                Some(version) => version.to_string(),
                None => "?".to_string()
            }
        });

        println!("\n Voice packages:");

        for package in game.voice_packages().available().unwrap() {
            println!(" - {} : {}", package.locale.to_name(), {
                match package.installed() {
                    true => "installed",
                    false => "available"
                }
            });
        }

        println!("");

        true
    }
}
