use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::*;
use cli_table::{Cell, Table, print_stdout};

use crate::lib::config;
use crate::lib::output::*;
use crate::lib::format_size;

pub struct VoiceInfo {
    args: Vec<Box<dyn Argument>>
}

impl VoiceInfo {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--game", vec!["--path", "-g", "-p"], true)
            ]
        })
    }
}

impl Command for VoiceInfo {
    fn get_name(&self) -> &str {
        "info"
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
                warn("You didn't specify the game path\n");

                // Stop command execution
                return false;
            }

            Game::new(config.paths.game)
        };

        let latest_version = Game::try_get_latest_version().expect("Failed to get latest game version");

        let mut table = vec![];

        // List installed packages

        for package in game.get_voice_packages().expect("Failed to get voice packages") {
            table.push(vec![
                "[X]".light_green().cell(),
                package.locale().to_name().light_green().cell(),
                format!("{} GB", format_size(package.get_size())).cell(),
                match package.try_get_version() {
                    Some(version) => {
                        if version == latest_version {
                            // Latest version
                            version.to_string()
                        } else {
                            // Outdated version
                            version.to_string().light_red().to_string()
                        }
                    },
                    None => "failed to get".light_red().to_string()
                }.cell()
            ]);
        }

        // List not installed packages

        for package in VoicePackage::list_latest().expect("Failed to list voice packages") {
            if !package.is_installed_in(game.path()) {
                table.push(vec![
                    "[ ]".cell(),
                    package.locale().to_name().cell(),
                    format!("{} GB", format_size(package.get_size())).cell(),
                    match package.try_get_version() {
                        Some(version) => version.to_string(),
                        None => "failed to get".light_red().to_string()
                    }.cell()
                ]);
            }
        }

        let table = table.table().title(vec![" I", "Name", "Size", "Version"]);

        print_stdout(table);

        true
    }
}
