use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Table};
use dialoguer::Input;
use indicatif::ProgressBar;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};
use ureq::Response;

// Args
#[derive(Parser)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Login and store token
    Login,
    /// Get courses for this semester
    Courses,
    /// Fetch assignments that aren't completed yet
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
        Ok(dirs.config_dir.join("canvas.json"))
    }

    fn read() -> Result<Self> {
        let path = Self::path()?;
        let config = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config)?)
    }

    fn write(&self) -> Result<()> {
        let path = Self::path()?;
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
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

fn load_with_message<T, U: FnOnce() -> Result<T>, V: ToString>(
    message: V,
    function: U,
) -> Result<T> {
    let spinner = ProgressBar::new_spinner().with_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(10));
    let result = function()?;
    spinner.finish_and_clear();
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Cache {
    time: DateTime<Utc>,
    courses: Option<Vec<Course>>,
    output: Option<Vec<Output>>,
}

impl Cache {
    fn new() -> Self {
        let cache = Cache::read().unwrap_or_default();
        let time = chrono::offset::Utc::now();
        Self { time, ..cache }
    }

    fn get() -> Result<Self> {
        let cache = Cache::read()?;
        let now = chrono::offset::Utc::now();
        if (now - cache.time).num_minutes() >= 10 {
            Err(anyhow!("Cache invalidated"))
        } else {
            Ok(cache)
        }
    }

    fn path() -> Result<PathBuf> {
        let dirs = AppDirs::new(Some("canvas"), false).ok_or(anyhow!("Invalid cache"))?;
        std::fs::create_dir_all(&dirs.config_dir)?;
        Ok(dirs.config_dir.join("cache.json"))
    }

    fn read() -> Result<Self> {
        let path = Self::path()?;
        let config = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config)?)
    }

    fn write(&self) -> Result<()> {
        let path = Self::path()?;
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

// Types
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Course {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Assignment {
    id: usize,
    name: String,
    due_at: Option<DateTime<Utc>>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Output {
    course_name: String,
    assignment_name: String,
    date: Option<NaiveDate>,
    days_left: i64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Login => {
            // Open token settings page in browser
            open::that("https://uia.instructure.com/profile/settings#access_tokens_holder")?;

            // Store token in configuration file
            let token = Input::<String>::new()
                .with_prompt("Token")
                .interact_text()?;
            Config { token }.write()?;
            println!("Wrote settings to {}", Config::path()?.display());
        }
        Command::Courses => {
            // Get current favorite courses
            let courses = match Cache::get().map(|it| it.courses) {
                Ok(Some(courses)) => courses,
                _ => {
                    let courses = load_with_message("Loading courses...", || {
                        Ok(fetch("users/self/favorites/courses")?.into_json::<Vec<Course>>()?)
                    })?;
                    let mut cache = Cache::new();
                    cache.courses = Some(courses.clone());
                    cache.write()?;
                    courses
                }
            };

            // Create table
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec![Cell::new("Course")]);
            for Course { name, .. } in courses {
                table.add_row(vec![Cell::new(name)]);
            }
            println!("{table}");
        }
        Command::Assignments => {
            // Get assignments from each course
            let output = match Cache::get().map(|it| it.output) {
                Ok(Some(output)) => output,
                _ => {
                    let courses = load_with_message("Loading courses...", || {
                        Ok(fetch("users/self/favorites/courses")?.into_json::<Vec<Course>>()?)
                    })?;
                    let mut output = Vec::new();
                    for Course { id, name } in courses {
                        let assignments = load_with_message(
                            format!("Loading assignments for {name}..."),
                            || {
                                let route = format!("courses/{id}/assignments");
                                eprintln!("{}", fetch(&route)?.into_string()?);
                                Ok(fetch(&route)?.into_json::<Vec<Assignment>>()?)
                            },
                        )?;
                        let course_name = name;
                        let outputs = assignments
                            .into_iter()
                            .map(|Assignment { name, due_at, .. }| {
                                let now = chrono::offset::Utc::now();
                                let date = due_at.map(|it| it.date_naive());
                                let days_left = match due_at {
                                    Some(due_at) => (due_at - now).num_days(),
                                    None => 0,
                                };
                                Output {
                                    course_name: course_name.clone(),
                                    assignment_name: name,
                                    date,
                                    days_left,
                                }
                            })
                            .filter(|output| output.days_left >= 0);
                        output.extend(outputs);
                    }
                    output.sort_by_key(|it| it.days_left);

                    // Store to cache then return
                    let mut cache = Cache::new();
                    cache.output = Some(output.clone());
                    cache.write()?;
                    output
                }
            };

            // Create table
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec![
                    Cell::new("Course"),
                    Cell::new("Name"),
                    Cell::new("Date"),
                    Cell::new("Days left"),
                ]);
            for Output {
                course_name,
                assignment_name,
                date,
                days_left,
            } in output
            {
                table.add_row(vec![
                    Cell::new(course_name),
                    Cell::new(assignment_name),
                    date.map(Cell::new).unwrap_or(Cell::new("")),
                    if days_left == 0 { Cell::new("") } else { Cell::new(days_left) },
                ]);
            }
            println!("{table}");
        }
    }

    Ok(())
}
