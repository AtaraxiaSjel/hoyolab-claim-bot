#![forbid(unsafe_code)]
use anyhow::Result;
use claim::claim;
use config::Config;
use std::env;

mod claim;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    claim(&config.users)?;
    Ok(())
}
