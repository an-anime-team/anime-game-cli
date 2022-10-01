use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::genshin::*;

use crate::lib::config;
use crate::lib::output::*;

pub struct PatchApply {
    args: Vec<Box<dyn Argument>>
}

impl PatchApply {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Flag::with_name("--no-root")
            ]
        })
    }
}

impl Command for PatchApply {
    fn get_name(&self) -> &str {
        "apply"
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

        let mut no_root = false;

        for arg in args {
            match arg.value.as_str() {
                "--no-root" => no_root = true,
                _ => unreachable!()
            }
        }

        let patch = PatchApplier::new(config.paths.patch);

        match patch.is_sync(&config.patch.hosts) {
            // Local patch is synced
            Ok(true) => {
                notice("Fetching latest patch info...");

                match Patch::try_fetch(config.patch.hosts, None) {
                    // Successfully fetched latest patch info
                    Ok(patch_info) => {
                        match patch_info.is_applied(&config.paths.game) {
                            // Patch is not applied to the game
                            Ok(false) => {
                                notice("Applying patch...");

                                match patch.apply(config.paths.game, patch_info, !no_root) {
                                    Ok(_) => notice("Patch successfully applied"),
                                    Err(err) => error(format!("Failed to apply patch: {}", err))
                                }
                            },
                            Ok(true) => notice("Patch is already applied"),
                            Err(err) => error(format!("Failed to check game patching status: {}", err)),
                        }
                    },
                    Err(err) => error(format!("Failed to fetch latest patch info: {}", err))
                }
            },
            Ok(false) => warn(format!("Patch is not synced. Run {} first", "patch sync".light_yellow())),
            Err(err) => error(format!("Failed to check patch folder: {}", err))
        }

        true
    }
}
