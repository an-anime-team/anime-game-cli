use commandor::prelude::*;

pub mod commands;
pub mod lib;

fn main() {
    let manager = Manager::new(vec![
        commands::Info::new(),
        commands::Voice::new(),
        commands::Patch::new()
    ]);

    println!("\n Anime Game CLI\n");

    match manager.execute(std::env::args().skip(1).collect()) {
        Ok(_) => (),
        
        Err(Error::TooFewArguments) => eprintln!("Arguments required"),
        Err(Error::CommandNotFound(command)) => eprintln!("Command {} not found", command),
        Err(Error::ArgumentRequired(argument)) => eprintln!("Argument {} required", argument)
    }
}
