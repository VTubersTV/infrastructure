use anyhow::Result;
use dialoguer::Input;
use git2::Repository;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use crate::config::DockerConfig;
use crate::utils::TempDirManager;

/// Manages Docker configurations and operations
pub struct DockerManager {
    temp_dir_manager: TempDirManager,
}

impl DockerManager {
    /// Creates a new DockerManager instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            temp_dir_manager: TempDirManager::new()?,
        })
    }

    /// Fetches Docker configurations from either local directory or GitHub repository
    pub async fn fetch_configs(&self, git_url: &str) -> Result<Vec<DockerConfig>> {
        let local_path = PathBuf::from("docker");
        
        let configs = if local_path.exists() {
            self.read_local_configs(&local_path)?
        } else {
            let temp_dir = self.clone_repository(git_url)?;
            self.temp_dir_manager.set_temp_dir(temp_dir.clone())?;
            self.read_local_configs(&temp_dir)?
        };
        
        Ok(configs)
    }

    /// Reads Docker configurations from a local directory
    fn read_local_configs(&self, path: &Path) -> Result<Vec<DockerConfig>> {
        let mut configs = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let env_vars = self.find_env_vars(&path)?;
                    configs.push(DockerConfig::new(
                        name.to_string(),
                        path,
                        env_vars,
                    ));
                }
            }
        }
        Ok(configs)
    }

    /// Finds environment variables in Docker configuration files
    fn find_env_vars(&self, path: &Path) -> Result<Vec<String>> {
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
                        .and_then(|s| s.split(':').next())
                        .map(|s| s.trim().to_string());
                    if let Some(var) = var {
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
                    if !built_in_vars.contains(&key.as_str()) {
                        env_vars.push(key);
                    }
                }
            }
        }

        env_vars.dedup();
        Ok(env_vars)
    }

    /// Clones a Git repository to a temporary directory
    fn clone_repository(&self, url: &str) -> Result<PathBuf> {
        let temp_dir = self.temp_dir_manager.get_temp_dir()?;
        
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        println!("{}", console::style("Cloning repository...").cyan());
        Repository::clone(url, &temp_dir)?;
        Ok(temp_dir)
    }

    /// Creates an environment file with user-provided values
    pub fn create_env_file(&self, path: &Path, env_vars: &[String]) -> Result<()> {
        let env_path = path.join(".env");
        let mut file = File::create(env_path)?;
        
        for var in env_vars {
            let value = Input::<String>::new()
                .with_prompt(format!("Enter value for {}", var))
                .interact()?;
            writeln!(file, "{}={}", var, value)?;
        }
        Ok(())
    }
} 