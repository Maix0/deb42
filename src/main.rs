use clap::Parser;
use color_eyre::eyre::Result;
use config::Config;
use eyre::OptionExt;
use figment::providers::Format;
use once_cell::sync::Lazy;
use std::{ops::Deref, sync::OnceLock};
#[macro_use]
extern crate log;

mod cmd;
mod config;
mod database;
mod logging;

static CONFIG: OnceLock<config::Config> = OnceLock::new();
static ARGS: Lazy<cmd::Command> = Lazy::new(|| cmd::Command::parse());
static DIRS: Lazy<directories::ProjectDirs> = Lazy::new(|| {
    directories::ProjectDirs::from("me", "maix", "42deb")
        .expect("Unable to get the project directories (how????)")
});

async fn setup_stuff() -> Result<()> {
    color_eyre::install()?;
    let _ = ARGS.deref(); // Initialize the arguments before doing anything;
    logging::set_logger()?;
    CONFIG
        .set(
            figment::Figment::new()
                .merge(figment::providers::Serialized::defaults(Config {
                    output_path: DIRS.data_dir().to_path_buf(),
                }))
                .merge(figment::providers::Toml::file(&ARGS.config_path))
                .extract()?,
        )
        .unwrap();
    tokio::fs::create_dir_all(
        &CONFIG
            .get()
            .ok_or_eyre("Unable to get config...")?
            .output_path,
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_stuff().await?;
    database::open_database().await?;
    debug!(
        "OUTPUT path = {}",
        CONFIG.get().unwrap().output_path.display()
    );
    Ok(())
}
