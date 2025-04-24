use std::process::Command;
use anyhow::{Result, Context};
use ctrlc;

fn main() -> Result<()> {
    start_redis()?;

    ctrlc::set_handler(move || {
        if let Err(e) = stop_redis() {
            eprintln!("Failed to stop Redis via brew: {}", e);
            std::process::exit(1);
        }
        std::process::exit(0);
    })?;

    loop {
        std::thread::park();
    }
}

fn start_redis() -> Result<()> {
    Command::new("brew")
        .args(["services", "start", "redis"])
        .status()
        .context("Failed to start Redis")?;

    Ok(())
}

fn stop_redis() -> Result<()> {
    Command::new("brew")
        .args(["services", "stop", "redis"])
        .status()
        .context("Failed to stop Redis")?;
    Ok(())
}