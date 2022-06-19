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
    pub paths: Paths
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: Paths {
                game: String::new()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paths {
    pub game: String
}
