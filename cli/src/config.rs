use std::path::PathBuf;

/// Represents a Docker configuration with its associated metadata
#[derive(Debug)]
pub struct DockerConfig {
    /// Name of the Docker configuration
    pub name: String,
    /// Path to the Docker configuration directory
    pub path: PathBuf,
    /// List of environment variables required by this configuration
    pub env_vars: Vec<String>,
}

impl DockerConfig {
    /// Creates a new DockerConfig instance
    pub fn new(name: String, path: PathBuf, env_vars: Vec<String>) -> Self {
        Self {
            name,
            path,
            env_vars,
        }
    }
} 