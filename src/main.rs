use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use ureq::Response;
use std::path::PathBuf;
use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS};

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

// Types
#[derive(Debug, Serialize, Deserialize)]
struct Course {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Assignment {
    id: usize,
    name: String,
    due_at: DateTime<Utc>,
    description: Option<String>,
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
            let courses = fetch("users/self/favorites/courses")?.into_json::<Vec<Course>>()?;
            for Course { id, name } in courses {
                let route = format!("courses/{id}/assignments");
                let assignments = fetch(&route)?.into_json::<Vec<Assignment>>()?;
                if assignments.len() == 0 {
                    continue;
                }
                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_header(vec![
                        name.as_str(),
                        "Date",
                        "Days left",
                    ]);
                for Assignment { name, due_at, .. } in assignments {
                    let now = chrono::offset::Utc::now();
                    let date = due_at.date_naive();
                    let delta = (due_at - now).num_days();
                    if delta >= 0 {
                        table.add_row(vec![
                            name,
                            format!("{date}"),
                            format!("{delta}"),
                        ]);
                    } 
                }
                println!("{table}");
            }
        }
    }

    Ok(())
}
