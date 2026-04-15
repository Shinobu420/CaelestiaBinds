mod cli;
mod hyprland;

fn main() {
    match cli::parse_args() {
        cli::CliAction::RunGui => {
            println!("Initializing CaelestiaBinds...");
        }
    }
}
