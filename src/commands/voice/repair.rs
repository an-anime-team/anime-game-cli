use std::io::Error;

use commandor::prelude::*;

use anime_game_core::voice_data::locale::VoiceLocale;
use anime_game_core::voice_data::package::VoicePackage;
use anime_game_core::repairer::*;

use crate::lib::output::*;
use crate::lib::config;
use crate::lib::command_traits::repair::*;

pub struct VoiceRepair {
    args: Vec<Box<dyn Argument>>
}

impl RepairFiles for VoiceRepair {
    fn try_get_integrity_files(args: Vec<String>) -> Result<Vec<IntegrityFile>, Error> {
        let config = config::get().expect("Failed to get config");

        let mut files = Vec::new();
        let mut locales = Vec::new();
        
        for arg in &args[1..] {
            if let Some(locale) = VoiceLocale::from_str(arg) {
                match VoicePackage::with_locale(locale.clone()) {
                    Some(package) => {
                        if !package.is_installed_in(&config.paths.game) {
                            warn(format!("{} package is not installed", locale.to_name()))
                        }

                        else {
                            locales.push(locale);

                            files.append(&mut try_get_voice_integrity_files(locale)?);
                        }
                    },
                    None => warn(format!("Failed to get {} package", locale.to_name()))
                }
            }
        }

        let locales = locales.into_iter().fold(String::new(), |acc, s| acc + s.to_name() + ", ");

        if locales.len() > 0 {
            notice(format!("Verifying locales: {}", &locales[..locales.len() - 2]));
        }

        Ok(files)
    }
}

impl VoiceRepair {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: Self::get_command_args()
        })
    }
}

impl Command for VoiceRepair {
    fn get_name(&self) -> &str {
        "repair"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, args: Vec<String>, values: Vec<ArgumentValue>) -> bool {
        Self::repair(RepairFilesConfig::from_args(values), args)
    }
}
