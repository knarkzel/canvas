use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Table};
use dialoguer::Input;
use indicatif::ProgressBar;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf, time::Duration};
use ureq::Response;

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

// Helpers
fn fetch(route: &str) -> Result<Response> {
    let Config { token } = Config::read()?;
    Ok(
        ureq::get(&format!("https://uia.instructure.com/api/v1/{route}"))
            .set("Authorization", &format!("Bearer {token}"))
            .call()?,
    )
}

fn load_with_message<T, U: FnOnce() -> Result<T>, V: ToString>(message: V, function: U) -> Result<T> {
    let spinner = ProgressBar::new_spinner().with_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(10));
    let result = function()?;
    spinner.finish_and_clear();
    Ok(result)
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
            let courses = load_with_message("Loading courses...", || {
                Ok(fetch("users/self/favorites/courses")?.into_json::<Vec<Course>>()?)
            })?;
            for Course { id, name } in courses {
                let assignments = load_with_message(format!("Loading assignments for {name}..."), || {
                    let route = format!("courses/{id}/assignments");
                    Ok(fetch(&route)?.into_json::<Vec<Assignment>>()?)
                })?;
                if assignments.len() == 0 {
                    continue;
                }
                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_header(vec![
                        Cell::new("Name"),
                        Cell::new("Date"),
                        Cell::new("Days left"),
                    ]);
                for Assignment { name, due_at, .. } in assignments {
                    let now = chrono::offset::Utc::now();
                    let date = due_at.date_naive();
                    let delta = (due_at - now).num_days();
                    if delta >= 0 {
                        table.add_row(vec![Cell::new(name), Cell::new(date), Cell::new(delta)]);
                    }
                }
                println!("{table}");
            }
        }
    }

    Ok(())
}
