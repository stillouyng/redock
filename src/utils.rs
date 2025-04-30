use std::process::Output;
use tokio::process::Command;

#[derive(Debug)]
pub enum Error {
    BrewNotFound,
    RedisCliNotFound,
    CommandFailed(String),
}

pub fn get_brew_path() -> Result<&'static str, Error> {
    let paths = [
        "/opt/homebrew/bin/brew",              // Apple Silicon
        "/usr/local/bin/brew",                 // Intel
        "/home/linuxbrew/.linuxbrew/bin/brew", // Linux
    ];

    for path in &paths {
        if std::path::Path::new(path).exists() {
            return Ok(path);
        }
    }
    Err(Error::BrewNotFound)
}

pub fn get_redis_cli_path() -> Result<&'static str, Error> {
    let paths = [
        "/opt/homebrew/bin/redis-cli", // Apple Silicon
        "/usr/local/bin/redis-cli",    // Intel
        "/usr/bin/redis-cli",          // System
    ];

    for path in &paths {
        if std::path::Path::new(path).exists() {
            return Ok(path);
        }
    }
    Err(Error::RedisCliNotFound)
}

pub async fn execute_command(command_name: &str, args: &[&str]) -> Result<Output, Error> {
    Command::new(command_name)
        .args(args)
        .output()
        .await
        .map_err(|e| Error::CommandFailed(e.to_string()))
}

pub fn format_output(result: Result<Output, Error>) -> String {
    match result {
        Ok(output) => {
            let status_icon = if output.status.success() { "✓" } else { "×" };
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

            match (stdout.is_empty(), stderr.is_empty()) {
                (true, true) => format!("{} command executed", status_icon),
                (false, true) => format!("{} {}", status_icon, stdout),
                (true, false) => format!("{} {}", status_icon, stderr),
                (false, false) => {
                    format!("{} Output: {}\nErrors: {}", status_icon, stdout, stderr)
                }
            }
        }
        Err(Error::BrewNotFound) => "🚨 Error: brew not found in standard locations".to_string(),
        Err(Error::RedisCliNotFound) => {
            "🚨 Error: redis-cli not found in standard locations".to_string()
        }
        Err(Error::CommandFailed(e)) => format!("🚨 Command failed: {}", e),
    }
}
