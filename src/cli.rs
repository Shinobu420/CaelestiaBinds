use clap::Parser;

// Overriding Clippy here to avoid adding unnecessary struct boilerplate.
#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "CaelestiaBinds")]
#[command(version, about, long_about = None)]
pub enum CliAction {
    RunGui,
}

pub fn parse_args() -> CliAction {
    CliAction::RunGui
}
