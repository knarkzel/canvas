use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use platform_dirs::AppDirs;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

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

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
}

fn config_path() -> Result<PathBuf> {
    let app_dirs = AppDirs::new(Some("canvas"), false).ok_or(anyhow!("Config location is invalid"))?;
    std::fs::create_dir_all(&app_dirs.config_dir)?;
    Ok(app_dirs.config_dir.join("canvas.toml"))
}

fn read_config() -> Result<Config> {
    let config_file = config_path()?;
    let raw = std::fs::read_to_string(config_file)?;
    Ok(toml::from_str(&raw)?)
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Login => {
            open::that("https://uia.instructure.com/profile/settings#access_tokens_holder")?;
            let token = Input::<String>::new()
                .with_prompt("Token")
                .interact_text()?;

            // Create config file
            let config_file = config_path()?;
            let config = Config { token };
            let toml = toml::to_string(&config)?;
            std::fs::write(&config_file, toml)?;
            println!("Wrote settings to {}", config_file.display());
        }
        Command::Assignments => {
            let Config { token } = read_config()?;
            todo!()
        },
    }

    Ok(())
}
