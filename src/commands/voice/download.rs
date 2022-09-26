use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use commandor::prelude::*;

use anime_game_core::prelude::*;
use anime_game_core::genshin::prelude::*;

use crate::lib::config;
use crate::lib::output::*;
use crate::lib::format_size;

pub struct VoiceDownload {
    args: Vec<Box<dyn Argument>>
}

impl VoiceDownload {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![]
        })
    }
}

impl Command for VoiceDownload {
    fn get_name(&self) -> &str {
        "download"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, args: Vec<String>, _: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        let game_path = {
            if config.paths.game == "" {
                error("Game path is not specified");

                // Interrupt command execution
                return false;
            }

            config.paths.game
        };

        let mut packages = HashSet::new();

        for arg in &args[1..] {
            match VoiceLocale::from_str(arg) {
                Some(locale) => match VoicePackage::with_locale(locale) {
                    Ok(package) => {
                        if package.is_installed_in(&game_path) {
                            // TODO: Check for updates
                            notice(format!("{} package is already installed", locale.to_name()))
                        }

                        else {
                            packages.insert(package);
                        }
                    },
                    Err(err) => warn(format!("Failed to get {} package: {}", locale.to_name(), err))
                },
                None => warn(format!("Failed to find \"{}\" language", arg))
            }
        }

        let progress = Arc::new(Mutex::new(linya::Progress::new()));
        let mut handlers = Vec::new();

        for package in packages {
            match package.try_get_diff() {
                Ok(diff) => {
                    let thread_progress = progress.clone();
                    let thread_game_path = game_path.clone();

                    handlers.push(std::thread::spawn(move || {
                        let (download_size, unpacked_size) = diff.size().unwrap();
                        
                        let downloading_bar = Arc::new(thread_progress.lock().unwrap().bar(
                            download_size as usize,
                            format!("{} ({} GB)", package.locale().to_name(), format_size(download_size))
                        ));

                        let unpacking_bar = Arc::new(thread_progress.lock().unwrap().bar(
                            unpacked_size as usize,
                            format!("{} ({} GB)", package.locale().to_name(), format_size(unpacked_size))
                        ));

                        let result = diff.install_to(thread_game_path, move |state| {
                            let mut thread_progress = thread_progress.lock().unwrap();

                            match state {
                                InstallerUpdate::CheckingFreeSpace(_) => (),
                                InstallerUpdate::DownloadingStarted(_) => (),
                                InstallerUpdate::DownloadingProgress(curr, _) => {
                                    thread_progress.set_and_draw(&downloading_bar, curr as usize);
                                },
                                InstallerUpdate::DownloadingFinished => {
                                    thread_progress.set_and_draw(&downloading_bar, download_size as usize);
                                },
                                InstallerUpdate::DownloadingError(_) => {
                                    // error("Failed to download package"); // todo
                                },
                                InstallerUpdate::UnpackingStarted(_) => (),
                                InstallerUpdate::UnpackingProgress(curr, _) => {
                                    thread_progress.set_and_draw(&unpacking_bar, curr as usize);
                                },
                                InstallerUpdate::UnpackingFinished => {
                                    thread_progress.set_and_draw(&unpacking_bar, unpacked_size as usize);
                                },
                                InstallerUpdate::UnpackingError(_) => {
                                    // error("Failed to unpack package"); // todo
                                }
                            }
                        });

                        if let Err(_) = result {
                            // todo
                        }
                    }));
                },
                Err(err) => error(format!("Failed to find difference for {} package: {}", package.locale().to_name(), err))
            }
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        true
    }
}
