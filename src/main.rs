mod helpers;
mod print;
mod scanner;
mod timeshift;
mod update;

use crate::helpers::{confirm_update, ensure_root};
use crate::print::{print_help, print_updates};
use crate::scanner::{get_pacman_updates_list, get_paru_updates_list};
use crate::timeshift::{create_timeshift_snapshot, delete_old_timeshift_snapshot};
use crate::update::{update_aur, update_pacman};
use anyhow::{Context, Result, anyhow};
use std::env;

struct Options {
    do_pacman: bool,
    do_aur: bool,
}

const COMMENT: &str = "update-system";

fn main() -> Result<()> {
    let opts = handle_args()?;

    if !opts.do_pacman && !opts.do_aur {
        println!("Both pacman and AUR updates disabled. Exiting.");
        return Ok(());
    }

    ensure_root().context("requires sudo/root")?;

    let pac_updates = get_pacman_updates_list(!opts.do_pacman)?;
    let aur_updates = get_paru_updates_list(!opts.do_aur)?;

    if pac_updates.is_empty() && aur_updates.is_empty() {
        println!("No pacman or AUR updates. Exiting.");
        return Ok(());
    }

    print_updates(opts.do_pacman, opts.do_aur, pac_updates, aur_updates);

    if !confirm_update()? {
        println!("Aborted by user.");
        return Ok(());
    }

    let newest = create_timeshift_snapshot(COMMENT)?;
    delete_old_timeshift_snapshot(COMMENT, &newest)?;

    if opts.do_pacman {
        update_pacman()?;
    }

    if opts.do_aur {
        update_aur()?;
    }

    return Ok(());
}

fn handle_args() -> Result<Options> {
    let mut do_pacman = true;
    let mut do_aur = true;
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "update-system".into());
    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help(&program);
                std::process::exit(0);
            }
            "--no-pacman" => do_pacman = false,
            "--no-aur" => do_aur = false,
            _ => return Err(anyhow!("unknown argument: {}", arg)),
        }
    }
    return Ok(Options { do_pacman, do_aur });
}
