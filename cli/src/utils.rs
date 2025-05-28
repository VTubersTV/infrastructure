use anyhow::Result;
use console::style;
use dirs::home_dir;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};
use ctrlc;

/// Manages temporary directory operations
pub struct TempDirManager {
    temp_dir: PathBuf,
}

impl TempDirManager {
    /// Creates a new TempDirManager instance
    pub fn new() -> Result<Self> {
        let temp_dir = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".docker-cli-temp");
        
        Ok(Self { temp_dir })
    }

    /// Gets the temporary directory path
    pub fn get_temp_dir(&self) -> Result<PathBuf> {
        Ok(self.temp_dir.clone())
    }

    /// Sets the temporary directory path
    pub fn set_temp_dir(&self, path: PathBuf) -> Result<()> {
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir_all(&path)?;
        Ok(())
    }

    /// Cleans up the temporary directory
    pub fn cleanup(&self) {
        if self.temp_dir.exists() {
            let _ = fs::remove_dir_all(&self.temp_dir);
        }
    }
}

/// Sets up the CTRL+C handler for graceful cleanup
pub fn setup_ctrlc_handler() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\n{}", style("Cleaning up...").yellow());
        std::process::exit(0);
    })?;

    Ok(())
} 
