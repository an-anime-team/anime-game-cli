use commandor::prelude::*;

pub mod info;
pub mod download;
pub mod repair;

pub struct Voice {
    args: Vec<Box<dyn Argument>>
}

impl Voice {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![]
        })
    }
}

impl Command for Voice {
    fn get_name(&self) -> &str {
        "voice"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, args: Vec<String>, _: Vec<ArgumentValue>) -> bool {
        let manager = Manager::new(vec![
            info::VoiceInfo::new(),
            download::VoiceDownload::new(),
            repair::VoiceRepair::new()
        ]);
    
        match manager.execute(args[1..].to_vec()) {
            Ok(_) => (),
            
            Err(Error::TooFewArguments) => eprintln!("Arguments required"),
            Err(Error::CommandNotFound(command)) => eprintln!("Command {} not found", command),
            Err(Error::ArgumentRequired(argument)) => eprintln!("Argument {} required", argument)
        }

        true
    }
}
