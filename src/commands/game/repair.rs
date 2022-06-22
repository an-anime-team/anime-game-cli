use std::sync::{Arc, Mutex};
use std::cmp::min;

use commandor::prelude::*;
use colorful::Colorful;

use anime_game_core::prelude::*;

use crate::lib::config;
use crate::lib::output::*;
use crate::lib::format_size;

pub struct GameRepair {
    args: Vec<Box<dyn Argument>>
}

impl GameRepair {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--threads", vec!["-t"], true), // Sets both --verify-threads and --repair-threads
                Default::new("--verify-threads", vec!["-vt"], true),
                Default::new("--repair-threads", vec!["-rt"], true),
                Setter::new("--ignore", vec!["-i", "--skip"], "=", true), // Case insensitive
                Flag::new("--verify", vec!["-v"]) // Verify only; don't repair
            ]
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

    fn execute(&self, _: Vec<String>, args: Vec<ArgumentValue>) -> bool {
        let config = config::get().expect("Failed to load config");

        let game_path = {
            if config.paths.game == "" {
                error("You didn't specify the game path\n");

                // Interrupt command execution
                return false;
            }

            config.paths.game
        };

        let mut verify_threads = 4;
        let mut repair_threads = 4;

        let mut ignore = vec!["UnityPlayer.dll", "crashreport.exe", "upload_crash.exe", "xlua.dll"];

        let mut just_verify = false;

        for arg in &args {
            match arg.name.as_str() {
                "--threads" => {
                    verify_threads = arg.value.parse::<usize>().expect("Wrong threads num");
                    repair_threads = verify_threads;
                },
                "--verify-threads" => verify_threads = arg.value.parse::<usize>().expect("Wrong threads num"),
                "--repair-threads" => repair_threads = arg.value.parse::<usize>().expect("Wrong threads num"),
                "--ignore" => ignore = arg.value.split(",").collect(),
                "--verify" => just_verify = true,
                _ => unreachable!()
            }
        }

        notice("Fetching integrity files...");

        // TODO: concurrent files repairing

        match repairer::try_get_integrity_files() {
            Ok(mut files) => {
                // Skip ignored files
                files = files.into_iter().filter(|file| {
                    for line in &ignore {
                        let path = file.path.to_lowercase();

                        if path.contains(line) {
                            return false;
                        }
                    }

                    true
                }).collect::<Vec<anime_game_core::repairer::IntegrityFile>>();

                // Don't try to run 4 threads for 1 file
                verify_threads = min(verify_threads, files.len());

                notice(format!("Verifying {} files in {} threads:\n", files.len(), verify_threads));

                let (broken_send, broken_recv) = std::sync::mpsc::channel();
                let mut handlers = Vec::new();

                let progress = Arc::new(Mutex::new(linya::Progress::new()));

                // Prepare threads
                let step = files.len() / verify_threads;

                for i in 0..verify_threads {
                    let files_part = Vec::from(if i == verify_threads - 1 {
                        &files[i * step..]
                    } else {
                        &files[i * step..(i + 1) * step]
                    });

                    let mut files_size = 0;

                    for file in &files_part {
                        files_size += file.size;
                    }

                    let game_path_ref = game_path.clone();

                    let thread_progress = progress.clone();
                    let thread_broken_send = broken_send.clone();

                    let bar = thread_progress.lock().unwrap().bar(files_part.len(), format!("Thread {} ({} GB)", i + 1, format_size(files_size)));

                    // Run thread
                    handlers.push(std::thread::spawn(move || {
                        for file in files_part {
                            if !file.verify(game_path_ref.clone()) {
                                thread_broken_send.send(file);
                            }

                            thread_progress.lock().unwrap().inc_and_draw(&bar, 1);
                        }
                    }));
                }

                // Sync threads
                for handler in handlers {
                    handler.join();
                }

                // Fetch broken files
                let mut broken_files = Vec::new();

                while let Ok(file) = broken_recv.try_recv() {
                    broken_files.push(file);
                }

                // Repair broken files

                println!();
                notice({
                    let mut output = vec![format!("Found {} broken files", broken_files.len())];

                    for file in &broken_files {
                        output.push(format!("- {}", file.path));
                    }

                    output
                });

                if !just_verify && broken_files.len() > 0 {
                    // Don't try to run 4 threads for 1 file
                    repair_threads = min(repair_threads, broken_files.len());
                    
                    println!();
                    notice(format!("Repairing {} files in {} threads:\n", broken_files.len(), repair_threads));

                    let (failed_send, failed_recv) = std::sync::mpsc::channel();
                    let mut handlers = Vec::new();

                    let progress = Arc::new(Mutex::new(linya::Progress::new()));

                    // Prepare threads
                    let step = files.len() / repair_threads;

                    for i in 0..repair_threads {
                        let files_part = Vec::from(if i == repair_threads - 1 {
                            &broken_files[i * step..]
                        } else {
                            &broken_files[i * step..(i + 1) * step]
                        });

                        let mut files_size = 0;

                        for file in &files_part {
                            files_size += file.size;
                        }

                        let game_path_ref = game_path.clone();

                        let thread_progress = progress.clone();
                        let thread_failed_send = failed_send.clone();

                        let bar = thread_progress.lock().unwrap().bar(files_part.len(), format!("Thread {} ({} GB)", i + 1, format_size(files_size)));

                        // Run thread
                        handlers.push(std::thread::spawn(move || {
                            for file in files_part {
                                if let Err(err) = file.repair(game_path_ref.clone()) {
                                    thread_failed_send.send((file, err));
                                }

                                thread_progress.lock().unwrap().inc_and_draw(&bar, 1);
                            }
                        }));
                    }

                    // Sync threads
                    for handler in handlers {
                        handler.join();
                    }

                    // Print failed to repair files
                    while let Ok((file, err)) = failed_recv.try_recv() {
                        error(format!("Failed to repair {}: {}", file.path, err));
                    }
                }
            },
            Err(err) => error(format!("Failed to get integrity files: {}", err))
        }

        true
    }
}
