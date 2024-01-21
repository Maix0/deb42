use crate::DIRS;
use std::path::PathBuf;
#[derive(Clone, Debug, clap::clap_derive::Parser)]
#[command(author = "maiboyer (aka Maix)", version = "0.0.1-alpha", about = "Install .deb packages without having root permission", long_about = None)]
pub struct Command {
    #[arg(default_value_os_t = DIRS.config_dir().to_path_buf(), long = "config", short = 'c')]
    /// This sets the location of the config directory
    pub config_path: PathBuf,
    #[command(subcommand)]
    pub command: Subcommands,
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    /// Enable verbose output
    /// you can stack the flag to enable even more verbose output to a limit
    pub verbose: u8,
}
#[derive(Clone, Debug, clap::clap_derive::Subcommand)]

pub enum Subcommands {
    Install(Install),
    DumpConfig(DumpConfig)

}

#[derive(Clone, Debug, clap::clap_derive::Parser)]
/// Install a .deb package providing its name as present into the selected registry
pub struct Install {
    #[arg(required = true, help = "List of package to be installed")]
    pub packages_name: Vec<String>
}

#[derive(Clone, Debug, clap::clap_derive::Parser)]
/// Dumps the current configuration onto stdout (formatted as TOML)
pub struct DumpConfig {}