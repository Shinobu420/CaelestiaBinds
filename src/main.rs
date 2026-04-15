mod cli;
mod config;
mod hyprland;
mod ui;

#[cfg(test)]
mod tests;

fn main() {
    match cli::parse_args() {
        cli::CliAction::RunGui => {
            println!("Initializing CaelestiaBinds...");
        }
    }
}
