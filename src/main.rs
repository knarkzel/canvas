use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Args
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Login and store token
    Login,
    /// Fetch assignments from Canvas
    Assignments,
}

// Config
#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
}

impl Config {
    fn path() -> Result<PathBuf> {
        let dirs = AppDirs::new(Some("canvas"), false).ok_or(anyhow!("Invalid config"))?;
        std::fs::create_dir_all(&dirs.config_dir)?;
        Ok(dirs.config_dir.join("canvas.toml"))
    }

    fn read() -> Result<Config> {
        let config_file = Config::path()?;
        let raw = std::fs::read_to_string(config_file)?;
        Ok(toml::from_str(&raw)?)
    }

    fn write(self) -> Result<()> {
        let config_file = Config::path()?;
        let toml = toml::to_string(&self)?;
        std::fs::write(&config_file, toml)?;
        println!("Wrote settings to {}", config_file.display());
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Login => {
            open::that("https://uia.instructure.com/profile/settings#access_tokens_holder")?;
            let token = Input::<String>::new()
                .with_prompt("Token")
                .interact_text()?;
            Config { token }.write()?;
            println!("Wrote settings to {}", Config::path()?.display());
        }
        Command::Assignments => {
            let Config { token } = Config::read()?;
            todo!()
        }
    }

    Ok(())
}
