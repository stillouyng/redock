use std::path::PathBuf;
use std::process::{Command, Output};

pub fn get_brew_path() -> &'static str {
    if std::path::Path::new("/opt/homebrew/bin/brew").exists() {
        "/opt/homebrew/bin/brew"
    } else {
        "/usr/local/bin/brew"
    }
}

pub fn get_redis_cli_path() -> &'static str {
    
    let paths = [
        "/opt/homebrew/bin/redis-cli",    // Apple Silicon
        "/usr/local/bin/redis-cli",       // Intel
        "/usr/bin/redis-cli"              // System
    ];

    for path in &paths {
        if std::path::Path::new(path).exists() {
            return path;
        }
    }
    "redis-cli"
}

pub fn execute_command(command_name: &str, args: &[&str]) -> Result<Output, String> {
    let output = Command::new(command_name)
        .args(args)
        .output()
        .map_err(|e| format!("Unexpected error: {}", e))?;

    Ok(output)
}

pub fn format_output(result: Result<Output, String>) -> String {
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
        Err(e) => format!("🚨 {}", e),
    }
}
