use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use ureq::Response;
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
        let path = Config::path()?;
        let config = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    }

    fn write(&self) -> Result<()> {
        let path = Config::path()?;
        std::fs::write(&path, toml::to_string(self)?)?;
        Ok(())
    }
}

// Helpers
fn fetch(route: &str) -> Result<Response> {
    let Config { token } = Config::read()?;
    Ok(ureq::get(&format!("https://uia.instructure.com/api/v1/{route}"))
       .set("Authorization", &format!("Bearer {token}"))
       .call()?)
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
            let output = fetch("users/self/favorites/courses")?.into_string()?;
            println!("{output}");
        }
    }

    Ok(())
}
