use std::collections::HashMap;
use std::{fs::File, io::Read};
use std::path::Path;
use std::io::{Error, ErrorKind, Write};

use serde::{Serialize, Deserialize};

pub const CONFIG_FILE: &'static str = "config.toml";

pub fn get() -> Result<Config, Error> {
    // Try to read config if the file exists
    if Path::new(CONFIG_FILE).exists() {
        let mut file = File::open(CONFIG_FILE)?;
        let mut toml = String::new();

        file.read_to_string(&mut toml)?;

        match toml::from_str::<Config>(&toml) {
            Ok(toml) => Ok(toml),
            Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to decode data from toml format: {}", err.to_string())))
        }
    }

    // Otherwise create default config file
    else {
        update(Config::default())?;

        Ok(Config::default())
    }
}

pub fn update(config: Config) -> Result<(), Error> {
    let mut file = File::create(CONFIG_FILE)?;

    match toml::to_string(&config) {
        Ok(toml) => {
            file.write_all(&mut toml.as_bytes())?;

            Ok(())
        },
        Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to encode data into toml format: {}", err.to_string())))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub paths: Paths,
    pub patch: Patch,
    pub wine: Wine
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: Paths {
                game: String::new()
            },
            patch: Patch {
                hosts: vec![
                    String::from("https://notabug.org/Krock/dawn")
                ]
            },
            wine: Wine {
                prefix: String::new(),
                executable: String::new(),
                environment: HashMap::new()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paths {
    pub game: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
    pub hosts: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wine {
    pub prefix: String,
    pub executable: String,
    pub environment: HashMap<String, String>
}
