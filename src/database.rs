use std::sync::{Mutex, OnceLock};

use color_eyre::eyre::Result;
use eyre::OptionExt;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use sqlx::SqlitePool;

pub static DATABASE: OnceLock<SqlitePool> = OnceLock::new();

pub async fn open_database() -> Result<()> {
    DATABASE
        .set(
            SqlitePool::connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename({
                        let mut p = crate::CONFIG.get().unwrap().output_path.clone();
                        p.push("installed.sqlite");
                        p
                    })
                    .create_if_missing(true)
                    .optimize_on_close(true, None),
            )
            .await?,
        )
        .unwrap();
    sqlx::migrate!().run(DATABASE.get().ok_or_eyre("Database get error")?).await?;
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct InstalledPackage {
    id: u64,
    name: String,
    depedencies: Vec<String>,
    version: String,
}
