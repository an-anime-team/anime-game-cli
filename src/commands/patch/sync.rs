use commandor::prelude::*;

use anime_game_core::prelude::*;

use crate::lib::config;
use crate::lib::output::*;

pub struct PatchSync {
    args: Vec<Box<dyn Argument>>
}

impl PatchSync {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Flag::new("--recursive", vec!["-r"])
            ]
        })
    }
}

impl Command for PatchSync {
    fn get_name(&self) -> &str {
        "sync"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, args: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        if config.patch.hosts.len() == 0 {
            error("Missing patch hosts");

            return false;
        }

        // Try to sync with all available repos until it's not succeeded
        let mut recursive = false;

        for arg in args {
            match arg.value.as_str() {
                "--recursive" => recursive = true,
                _ => unreachable!()
            }
        }

        let patch = PatchApplier::new(config.paths.patch);

        match patch.is_sync(&config.patch.hosts) {
            Ok(true) => notice("Patch is already synced"),
            Ok(false) => {
                let hosts = if recursive { &config.patch.hosts } else { &config.patch.hosts[..1] };

                notice("Syncing patch...");

                for host in hosts {
                    match patch.sync(host) {
                        Ok(true) => {
                            notice("Patch successfully synced");
    
                            return true;
                        },
                        Ok(false) => warn(format!("Failed to sync repo {}", host)),
                        Err(err) => warn(format!("Failed to sync repo {}: {}", host, err.to_string()))
                    }
                }

                error("Failed to sync patch");
            },
            Err(err) => error(format!("Failed to check patch folder: {}", err))
        }

        true
    }
}
