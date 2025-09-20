use anyhow::{Context, Result, anyhow};
use std::process::Command;

pub fn get_pacman_updates_list(opted_out: bool) -> Result<String> {
    if opted_out {
        return Ok(String::new());
    }

    let out = Command::new("pacman").args(["-Qu"]).output()?;
    let result = String::from_utf8_lossy(&out.stdout)
        .to_string()
        .trim()
        .to_string();
    if !out.status.success() && !result.is_empty() {
        return Err(anyhow!("pacman -Qu failed"));
    }
    return Ok(result);
}

pub fn get_paru_updates_list(opted_out: bool) -> Result<String> {
    if opted_out {
        return Ok(String::new());
    }

    let out = Command::new("paru")
        .args(["-Qua"])
        .output()
        .context("paru -Qua")?;
    return Ok(String::from_utf8_lossy(&out.stdout).to_string());
}
