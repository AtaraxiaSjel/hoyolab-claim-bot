use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]

pub enum TomlError {
    #[error("Couldn't read config file")]
    Read,
    #[error("Couldn't parse config file")]
    Parse,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub ltuid: String,
    pub ltoken: String,
    pub username: Option<String>,
    #[serde(default)]
    pub genshin: bool,
    #[serde(default)]
    pub honkai: bool,
    #[serde(rename(deserialize = "star-rail"))]
    #[serde(default)]
    pub star_rail: bool,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub users: Vec<User>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config> {
        let filename = match args.get(1) {
            Some(f) => f,
            None => "config.toml",
        };
        let contents = fs::read_to_string(filename).map_err(|_| TomlError::Read)?;
        let config: Config = toml::from_str(&contents).map_err(|_| TomlError::Parse)?;
        Ok(config)
    }
}
