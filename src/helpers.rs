use anyhow::{Result, anyhow};
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn ensure_root() -> Result<()> {
    let out = Command::new("id")
        .arg("-u")
        .stdout(Stdio::piped())
        .output()?;
    let uid = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if uid != "0" {
        return Err(anyhow!("not running as root. run with: sudo update-system"));
    } else {
        return Ok(());
    }
}

pub fn confirm_update() -> Result<bool> {
    print!("Proceed to create Timeshift snapshot and update? [Y/n]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let a = input.trim();
    return Ok(a.is_empty() || a.eq_ignore_ascii_case("y") || a.eq_ignore_ascii_case("yes"));
}
