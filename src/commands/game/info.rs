use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::*;
use anime_game_core::genshin::prelude::*;

use crate::lib::config;
use crate::lib::output::*;
use crate::lib::format_size;

pub struct GameInfo {
    args: Vec<Box<dyn Argument>>
}

impl GameInfo {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![]
        })
    }
}

impl Command for GameInfo {
    fn get_name(&self) -> &str {
        "info"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, _: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        let game = {
            if config.paths.game == "" {
                notice("You didn't specify the game path\n");
            }

            Game::new(config.paths.game)
        };

        match game.try_get_diff() {
            Ok(VersionDiff::NotInstalled { latest, .. }) => {
                warn(vec![
                    "Game is not installed".to_string(),
                    format!("Latest version: {}", latest.to_string().light_green())
                ]);

                // Interrupt command execution
                return true;
            },
            Ok(VersionDiff::Outdated { current, latest }) => {
                warn(vec![
                    "Your game installation is too outdated".to_string(),
                    format!("Current version: {}", current.to_string().light_red()),
                    format!("Latest version: {}", latest.to_string().light_green())
                ]);
            },
            Ok(VersionDiff::Latest(version)) => {
                notice(format!("Latest version: {}", version.to_string().light_green()));
            },
            Ok(VersionDiff::Diff { current, latest, unpacked_size, .. }) => {
                notice(vec![
                    format!(
                        "Game update available: {} -> {}",
                        current.to_string().light_yellow(),
                        latest.to_string().light_green()
                    ),
                    format!("Update size: {} GB", format_size(unpacked_size).to_string().light_cyan())
                ]);
            },
            Err(_) => todo!(),
        }

        true
    }
}
