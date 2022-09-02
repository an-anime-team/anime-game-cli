use commandor::prelude::*;

pub mod commands;
pub mod lib;

fn main() {
    let manager = Manager::new(vec![
        commands::Info::new(),
        commands::Voice::new(),
        commands::Patch::new(),
        commands::Game::new(),
        commands::Run::new(),
        commands::Help::new()
    ]);

    println!("\n Anime Game CLI\n");

    match manager.execute(std::env::args().skip(1).collect()) {
        Ok(_) => (),
        
        Err(Error::TooFewArguments) => eprintln!("Arguments required"),
        Err(Error::CommandNotFound(command)) => eprintln!("Command {} not found", command),
        Err(Error::ArgumentRequired(argument)) => eprintln!("Argument {} required", argument)
    }

    /*use anime_game_core::prelude::*;
    use anime_game_core::honkai::prelude::*;

    const HONKAI_PATH: &str = "/home/observer/.local/share/anime-game-launcher/game/drive_c/Program Files/Honkai Impact";
    const TEMP_PATH: &str = "/home/observer/.local/share/anime-game-launcher";

    let game = Game::new(HONKAI_PATH);

    dbg!(game.try_get_diff());

    let diff = game.try_get_diff().unwrap();

    let (send, recv) = std::sync::mpsc::channel();

    let th = std::thread::spawn(move || {
        let mut progress = linya::Progress::new();
        let bar = progress.bar(100000, "");

        while let Ok(fraction) = recv.try_recv() {
            progress.set_and_draw(&bar, fraction);
        }
    });

    diff.install_to_by(HONKAI_PATH, Some(TEMP_PATH), move |status| {
        match status {
            InstallerUpdate::CheckingFreeSpace(path) => println!("Checking free space in {path}..."),
            InstallerUpdate::DownloadingStarted(_) => println!("Downloading started"),
            InstallerUpdate::DownloadingFinished => println!("Downloading finished"),
            InstallerUpdate::DownloadingError(err) => println!("Downloading error: {:?}", err),
            InstallerUpdate::UnpackingStarted(_) => println!("Unpacking started"),
            InstallerUpdate::UnpackingFinished => println!("Unpacking finished"),
            InstallerUpdate::UnpackingError(err) => println!("Unpacking error: {err}"),

            InstallerUpdate::DownloadingProgress(curr, total) => {
                send.send((curr as f64 / total as f64 * 100000.0) as usize).unwrap();
            }

            InstallerUpdate::UnpackingProgress(curr, total) => {
                send.send((curr as f64 / total as f64 * 100000.0) as usize).unwrap();
            }
        }
    }).unwrap();

    th.join().unwrap();*/
}
