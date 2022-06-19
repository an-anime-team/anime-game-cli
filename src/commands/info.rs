use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::*;

use crate::lib::config;
use crate::lib::output::*;

/// Convert bytes to gigabytes with 2 digits round
fn format_size(bytes: u64) -> f64 {
    (bytes as f64 / 1024.0 / 1024.0 / 1024.0 * 100.0).ceil() / 100.0
}

pub struct Info {
    args: Vec<Box<dyn Argument>>
}

impl Info {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--game", vec!["--path", "-g", "-p"], true)
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
        let config = config::get().expect("Failed to load config");

        let game = if values.len() > 0 {
            Game::new(values[0].value.clone())
        } else {
            if config.paths.game == "" {
                notice("You didn't specify the game path\n");
            }

            Game::new(config.paths.game)
        };

        let mut latest_version = None;

        match game.try_get_diff() {
            Ok(GameVersionDiff::NotInstalled { latest, data: _, game_path: _ }) => {
                warn(vec![
                    "Game is not installed".to_string(),
                    format!("Latest version: {}", latest.to_string().light_green())
                ]);

                // Interrupt command execution
                return true;
            },
            Ok(GameVersionDiff::Outdated { current, latest }) => {
                warn(vec![
                    "Your game installation is too outdated".to_string(),
                    format!("Current version: {}", current.to_string().light_red()),
                    format!("Latest version: {}", latest.to_string().light_green())
                ]);

                latest_version = Some(latest);
            },
            Ok(GameVersionDiff::Latest(version)) => {
                notice(format!("Latest version: {}", version.to_string().light_green()));

                latest_version = Some(version);
            },
            Ok(GameVersionDiff::Diff { current, latest, data, game_path: _ }) => {
                notice(vec![
                    format!(
                        "Game update available: {} -> {}",
                        current.to_string().light_yellow(),
                        latest.to_string().light_green()
                    ),
                    format!("Update size: {} GB", format_size(data.size.parse::<u64>().unwrap()).to_string().light_cyan())
                ]);

                latest_version = Some(latest);
            },
            Err(_) => todo!(),
        }

        println!("\n Installed voice packages:");

        match game.get_voice_packages() {
            Ok(packages) => {
                for mut package in packages {
                    println!(" - {} ({} - {} GB)", package.locale().to_name(), {
                        match package.try_get_version() {
                            Some(version) => match latest_version {
                                Some(latest_version) => if version == latest_version {
                                    // version is latest
                                    version.to_string().light_green()
                                } else {
                                    // version is not latest
                                    version.to_string().light_red()
                                },
                                // latest version not known
                                None => version.to_string().light_yellow()
                            },
                            // failed to get latest version
                            None => "?".to_string().light_red()
                        }
                    }, {
                        format_size(package.get_size()).to_string().light_cyan()
                    });
                }
            },
            Err(err) => error(format!("Failed to get installed voice packages: {}", err.to_string()))
        }

        println!("\n Available voice packages:");

        match VoicePackage::list_latest() {
            Some(packages) => {
                for package in packages {
                    if !package.is_installed_in(game.path()) {
                        println!(" - {} ({} GB)", package.locale().to_name(), {
                            format_size(package.get_size()).to_string().light_cyan()
                        });
                    }
                }
            },
            None => error("Failed to get available voice packages")
        }

        true
    }
}
