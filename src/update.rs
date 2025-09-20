use anyhow::{Result, anyhow};
use std::process::Command;

pub fn update_pacman() -> Result<()> {
    let status = Command::new("pacman").args(["-Syu"]).status()?;
    if !status.success() {
        return Err(anyhow!("pacman -Syu failed"));
    }
    return Ok(());
}

pub fn update_aur() -> Result<()> {
    let status = Command::new("paru").args(["-Sua"]).status()?;
    if !status.success() {
        return Err(anyhow!("paru -Sua failed"));
    }
    return Ok(());
}
