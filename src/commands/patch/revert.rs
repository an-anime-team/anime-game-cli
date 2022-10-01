use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::genshin::*;

use crate::lib::config;
use crate::lib::output::*;

pub struct PatchRevert {
    args: Vec<Box<dyn Argument>>
}

impl PatchRevert {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Flag::new("--force", vec!["-f"])
            ]
        })
    }
}

impl Command for PatchRevert {
    fn get_name(&self) -> &str {
        "revert"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, args: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        if config.patch.hosts.is_empty() {
            error("Missing patch hosts");

            return false;
        }

        let mut force_revert = false;

        for arg in args {
            match arg.value.as_str() {
                "--force" => force_revert = true,
                _ => unreachable!()
            }
        }

        let applier = PatchApplier::new(&config.paths.patch);

        match applier.is_sync(&config.patch.hosts) {
            Ok(true) => {
                notice("Fetching latest patch info...");

                match Patch::try_fetch(config.patch.hosts.clone(), None) {
                    Ok(patch) => {
                        match patch {
                            Patch::NotAvailable |
                            Patch::Outdated { .. } |
                            Patch::Preparation { .. } => warn("Patch can't be reverted as it's not in stable nor testing stage"),
        
                            // Testing / Available
                            patch => {
                                match applier.revert(config.paths.game, patch, force_revert) {
                                    Ok(true) => notice("Patch reverted successfully"),
                                    Ok(false) => error("Failed to revert patch"),
                                    Err(err) => error(format!("Failed to revert patch: {}", err))
                                }
                            }
                        }
                    },
                    Err(err) => error(format!("Failed to fetch patch info: {}", err))
                }
            },
            Ok(false) => warn(format!("Patch is not synced. Run {} first", "patch sync".light_yellow())),
            Err(err) => error(format!("Failed to check patch folder: {}", err))
        }

        true
    }
}
