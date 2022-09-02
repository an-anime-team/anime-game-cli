use commandor::prelude::*;

use anime_game_core::genshin::repairer::try_get_integrity_files;
use anime_game_core::repairer::IntegrityFile;

use crate::lib::command_traits::repair::*;

pub struct GameRepair {
    args: Vec<Box<dyn Argument>>
}

impl RepairFiles for GameRepair {
    fn try_get_integrity_files(_: Vec<String>) -> anyhow::Result<Vec<IntegrityFile>> {
        try_get_integrity_files(None)
    }
}

impl GameRepair {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: Self::get_command_args()
        })
    }
}

impl Command for GameRepair {
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
