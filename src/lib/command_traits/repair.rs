use std::sync::{Arc, Mutex};
use std::cmp::min;
use std::io::Error;

use commandor::prelude::*;

use anime_game_core::repairer::*;

use crate::lib::config;
use crate::lib::output::*;
use crate::lib::format_size;

#[derive(Debug, Clone)]
pub struct RepairFilesConfig {
    pub verify_threads: usize,
    pub repair_threads: usize,
    pub ignore: Vec<String>,
    pub just_verify: bool,
    pub fast: bool
}

impl std::default::Default for RepairFilesConfig {
    fn default() -> Self {
        Self {
            verify_threads: 4,
            repair_threads: 4,
            ignore: vec![],
            just_verify: false,
            fast: false
        }
    }
}

impl RepairFilesConfig {
    pub fn from_args(args: Vec<ArgumentValue>) -> Self {
        let mut config = Self::default();

        for arg in &args {
            match arg.name.as_str() {
                "--threads" => {
                    config.verify_threads = arg.value.parse::<usize>().expect("Wrong threads num");
                    config.repair_threads = config.verify_threads;
                },
                "--verify-threads" => config.verify_threads = arg.value.parse::<usize>().expect("Wrong threads num"),
                "--repair-threads" => config.repair_threads = arg.value.parse::<usize>().expect("Wrong threads num"),
                "--ignore" => config.ignore = arg.value.split(",").map(|f| f.to_string()).collect(),
                "--verify" => config.just_verify = true,
                "--fast" => config.fast = true,
                _ => unreachable!()
            }
        }

        config
    }
}

fn calc_size(files: &Vec<anime_game_core::repairer::IntegrityFile>) -> u64 {
    let mut size = 0;

    for file in files {
        size += file.size;
    }

    size
}

pub trait RepairFiles {
    fn get_command_args() -> Vec<Box<dyn Argument>> {
        vec![
            Default::new("--threads", vec!["-t"], true), // Sets both --verify-threads and --repair-threads
            Default::new("--verify-threads", vec!["-vt"], true),
            Default::new("--repair-threads", vec!["-rt"], true),
            Setter::new("--ignore", vec!["-i", "--skip"], "=", true), // Case insensitive
            Flag::new("--verify", vec!["-v"]), // Verify only; don't repair
            Flag::new("--fast", vec!["-f"]) // Fast mode; compares files' sizes only
        ]
    }

    fn try_get_integrity_files(args: Vec<String>) -> Result<Vec<IntegrityFile>, Error>;

    fn repair(mut repairing_config: RepairFilesConfig, args: Vec<String>) -> bool {
        let config = config::get().expect("Failed to load config");

        let game_path = {
            if config.paths.game == "" {
                error("You didn't specify the game path\n");

                // Interrupt command execution
                return false;
            }

            config.paths.game
        };

        notice("Fetching integrity files...");

        match Self::try_get_integrity_files(args) {
            Ok(mut files) => {
                // Skip ignored files
                files = files.into_iter().filter(|file| {
                    for line in &repairing_config.ignore {
                        let path = file.path.to_lowercase();

                        if path.contains(line) {
                            return false;
                        }
                    }

                    true
                }).collect::<Vec<anime_game_core::repairer::IntegrityFile>>();

                if files.len() == 0 {
                    warn("No files found to verify");

                    return false;
                }

                // Don't try to run 4 threads for 1 file
                repairing_config.verify_threads = min(repairing_config.verify_threads, files.len());

                notice(format!("Verifying {} files ({} GB) in {} threads:\n", files.len(), format_size(calc_size(&files)), repairing_config.verify_threads));

                let (broken_send, broken_recv) = std::sync::mpsc::channel();
                let mut handlers = Vec::new();

                let progress = Arc::new(Mutex::new(linya::Progress::new()));

                // Prepare threads
                let average_thread_size = calc_size(&files) / repairing_config.verify_threads as u64;

                let mut i = 0;
                let mut j = 0;

                for _ in 0..repairing_config.verify_threads {
                    let mut files_part = Vec::new();
                    let mut files_part_size = 0;

                    while files_part_size < average_thread_size && i < files.len() {
                        files_part.push(files[i].clone());
                        files_part_size += files[i].size;

                        i += 1;
                    }

                    if files_part.len() > 0 {
                        let game_path_ref = game_path.clone();

                        let thread_progress = progress.clone();
                        let thread_broken_send = broken_send.clone();

                        j += 1;

                        let bar = thread_progress.lock().unwrap().bar(files_part.len(), format!("Thread {} ({} GB of {} files)", j, format_size(files_part_size), files_part.len()));

                        // Run thread
                        handlers.push(std::thread::spawn(move || {
                            for file in files_part {
                                let status = if repairing_config.fast {
                                    file.fast_verify(&game_path_ref)
                                } else {
                                    file.verify(&game_path_ref)
                                };

                                if !status {
                                    thread_broken_send.send(file);
                                }

                                thread_progress.lock().unwrap().inc_and_draw(&bar, 1);
                            }
                        }));
                    }
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

                if !repairing_config.just_verify && broken_files.len() > 0 {
                    // Don't try to run 4 threads for 1 file
                    repairing_config.repair_threads = min(repairing_config.repair_threads, broken_files.len());
                    
                    println!();
                    notice(format!("Repairing {} files in {} threads:\n", broken_files.len(), repairing_config.repair_threads));

                    let (failed_send, failed_recv) = std::sync::mpsc::channel();
                    let mut handlers = Vec::new();

                    let progress = Arc::new(Mutex::new(linya::Progress::new()));

                    // Prepare threads
                    let step = files.len() / repairing_config.repair_threads;

                    for i in 0..repairing_config.repair_threads {
                        let files_part = Vec::from(if i == repairing_config.repair_threads - 1 {
                            &broken_files[i * step..]
                        } else {
                            &broken_files[i * step..(i + 1) * step]
                        });

                        let game_path_ref = game_path.clone();

                        let thread_progress = progress.clone();
                        let thread_failed_send = failed_send.clone();

                        let bar = thread_progress.lock().unwrap().bar(files_part.len(), format!("Thread {} ({} GB of {} files)", i + 1, format_size(calc_size(&files_part)), files_part.len()));

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
                        error(format!("Failed to repair {}: {:?}", file.path, err));
                    }
                }
            },
            Err(err) => error(format!("Failed to get integrity files: {}", err))
        }

        true
    }
}
