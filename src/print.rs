pub fn print_help(program: &str) {
    println!(
        "Usage: {prog} [--no-pacman] [--no-aur]\n\
         \n\
         Options:\n\
           -h, --help     Show this help\n\
           --no-pacman    Skip pacman checks and updates\n\
           --no-aur       Skip AUR checks and updates",
        prog = program
    );
}

pub fn print_updates(do_pacman: bool, do_aur: bool, pac_updates: String, aur_updates: String) {
    if do_pacman {
        if pac_updates.trim().is_empty() {
            println!("Pacman updates: none");
        } else {
            println!("Pacman updates:");
            println!("{pac_updates}");
        }
    }
    if do_aur {
        if aur_updates.trim().is_empty() {
            println!("AUR updates: none");
        } else {
            println!("AUR updates:");
            println!("{aur_updates}");
        }
    }
}
