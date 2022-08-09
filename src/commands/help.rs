use commandor::prelude::*;

use crate::lib::output::*;

pub struct Help {
   args: Vec<Box<dyn Argument>> 
}

impl Help {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![]
        })
    }
}

impl Command for Help {
    fn get_name(&self) -> &str {
        "help"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, _:Vec<ArgumentValue>) -> bool {
        notice(vec![
            "Usage: anime-game-cli [game, patch, ...] [info, download, ...]",
            "",
            "game:",
            "├─ info: Get installed game info",
            "├─ download: Download the game (WIP)",
            "├─ update: Update the game (WIP)",
            "└─ repair: Repair the game",
            "",
            "voice:",
            "├─ info: List installed voice packages",
            "├─ download: Install additional voice package (WIP)",
            "├─ update: Update voice packages (WIP)",
            "├─ remove: Remove voice package (WIP)",
            "└─ repair: Repair voice packages",
            "",
            "patch:",
            "├─ info: Get info about the GNU/Linux patch",
            "├─ sync: Sync latest patch from remote repo",
            "├─ apply: Apply patch",
            "└─ revert: Revert patch",
            "",
            "info: Get info about the game, patch and voice packages",
            "run: Run the game",
            "help: Print this dialog",
            ""
        ]);

        true
    }
}

