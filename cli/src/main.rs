use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use git2::Repository;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};
use tokio;
use ctrlc;
use clap::Parser;

mod config;
mod docker;
mod utils;

use config::DockerConfig;
use docker::DockerManager;
use utils::TempDirManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Custom Git repository URL for Docker configurations
    #[arg(long, default_value = "https://github.com/VTubersTV/infrastructure")]
    git_url: String,
}

static TEMP_DIR: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

fn cleanup() {
    if let Ok(mut temp_dir) = TEMP_DIR.lock() {
        if let Some(path) = temp_dir.take() {
            if path.exists() {
                let _ = fs::remove_dir_all(&path);
            }
        }
    }
}

fn setup_ctrlc_handler() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\n{}", style("Cleaning up...").yellow());
        cleanup();
        std::process::exit(0);
    })?;

    Ok(())
}

async fn fetch_docker_configs() -> Result<Vec<DockerConfig>> {
    let github_url = "https://github.com/VTubersTV/infrastructure";
    let local_path = PathBuf::from("docker");
    let configs = if local_path.exists() {
        read_local_configs(&local_path)?
    } else {
        let temp_dir = clone_repository(github_url)?;
        // Store the temp directory path for cleanup
        if let Ok(mut temp_dir_guard) = TEMP_DIR.lock() {
            *temp_dir_guard = Some(temp_dir.clone());
        }
        read_local_configs(&temp_dir)?
    };
    Ok(configs)
}

fn read_local_configs(path: &Path) -> Result<Vec<DockerConfig>> {
    let mut configs = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let env_vars = find_env_vars(&path)?;
                configs.push(DockerConfig {
                    name: name.to_string(),
                    path,
                    env_vars,
                });
            }
        }
    }
    Ok(configs)
}

fn find_env_vars(path: &Path) -> Result<Vec<String>> {
    let mut env_vars = Vec::new();
    let docker_compose = path.join("docker-compose.yml");
    let env_file = path.join(".env");

    // List of Docker Compose built-in variables to ignore
    let built_in_vars = ["PWD", "HOME", "USER", "PATH", "HOSTNAME"];

    if docker_compose.exists() {
        let content = fs::read_to_string(docker_compose)?;
        for line in content.lines() {
            if line.contains("${") {
                let var = line
                    .split("${")
                    .nth(1)
                    .and_then(|s| s.split('}').next())
                    .and_then(|s| s.split(':').next()) // Split on : to remove default value
                    .map(|s| s.trim().to_string());
                if let Some(var) = var {
                    // Only add if it's not a built-in variable
                    if !built_in_vars.contains(&var.as_str()) {
                        env_vars.push(var);
                    }
                }
            }
        }
    }

    if env_file.exists() {
        let content = fs::read_to_string(env_file)?;
        for line in content.lines() {
            if let Some((key, _)) = line.split_once('=') {
                let key = key.trim().to_string();
                // Only add if it's not a built-in variable
                if !built_in_vars.contains(&key.as_str()) {
                    env_vars.push(key);
                }
            }
        }
    }

    // Remove duplicates while preserving order
    env_vars.dedup();
    Ok(env_vars)
}

fn clone_repository(url: &str) -> Result<PathBuf> {
    let temp_dir = home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".docker-cli-temp");
    
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    println!("{}", style("Cloning repository...").cyan());
    Repository::clone(url, &temp_dir)?;
    Ok(temp_dir)
}

fn create_env_file(path: &Path, env_vars: &[String]) -> Result<()> {
    let env_path = path.join(".env");
    let mut file = File::create(env_path)?;
    
    for var in env_vars {
        let value = dialoguer::Input::<String>::new()
            .with_prompt(format!("Enter value for {}", var))
            .interact()?;
        writeln!(file, "{}={}", var, value)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup CTRL+C handler
    utils::setup_ctrlc_handler()?;

    // Parse command line arguments
    let args = Args::parse();

    println!("{}", style("Docker Configuration Manager").cyan().bold());
    
    let docker_manager = DockerManager::new()?;
    let configs = docker_manager.fetch_configs(&args.git_url).await?;
    
    if configs.is_empty() {
        println!("{}", style("No Docker configurations found!").red());
        return Ok(());
    }

    let selections: Vec<String> = configs.iter().map(|c| c.name.clone()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Docker configuration")
        .items(&selections)
        .interact()?;

    let selected_config = &configs[selection];
    println!(
        "{}",
        style(format!("Selected: {}", selected_config.name)).green()
    );

    if !selected_config.env_vars.is_empty() {
        println!(
            "{}",
            style("Found environment variables that need to be configured:").yellow()
        );
        docker_manager.create_env_file(&selected_config.path, &selected_config.env_vars)?;
        println!(
            "{}",
            style("Environment file created successfully!").green()
        );
    }

    println!(
        "{}",
        style("Configuration is ready! You can now use docker-compose in the selected directory.").green()
    );
    println!(
        "{}",
        style(format!("Path: {}", selected_config.path.display())).cyan()
    );

    Ok(())
}