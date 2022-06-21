use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::*;

use crate::lib::config;
use crate::lib::output::*;

pub struct PatchInfo {
    args: Vec<Box<dyn Argument>>
}

impl PatchInfo {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![]
        })
    }
}

impl Command for PatchInfo {
    fn get_name(&self) -> &str {
        "info"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, _: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        match Patch::try_fetch(config.patch.hosts) {
            Ok(patch) => {
                match patch {
                    Patch::NotAvailable => error("Patch is not available"),
                    Patch::Outdated { current, latest, .. } => {
                        warn(vec![
                            String::from("Patch is outdated"),
                            format!("Patch version: {}", current),
                            format!("Latest version: {}", latest)
                        ]);
                    },
                    Patch::Preparation { .. } => warn("Patch is in preparation state"),

                    // Testing / Available
                    patch => {
                        notice(vec![
                            format!("Patch status: {}", {
                                if let Patch::Testing { .. } = patch {
                                    "testing".light_yellow()
                                } else {
                                    "stable".light_green()
                                }
                            }),
                            format!("Status {}", match patch.is_applied(config.paths.game) {
                                Ok(true) => "applied".light_green(),
                                Ok(false) => "not applied".light_red(),
                                Err(err) => format!("failed to check: {}", err).light_red()
                            })
                        ]);
                    }
                }
            },
            Err(err) => error(format!("Failed to fetch patch info: {}", err))
        }

        true
    }
}
