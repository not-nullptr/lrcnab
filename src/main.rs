use tokio::sync::Semaphore;

use crate::{client::LrcLib, watcher::Watcher};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

mod client;
mod entry;
mod scan;
mod watcher;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lrcnab=debug".into()),
        )
        .init();

    let client = LrcLib::new()?;
    let Some(path) = std::env::args().nth(1) else {
        tracing::error!("no path provided");
        std::process::exit(1);
    };

    let sem = Arc::new(Semaphore::new(16));

    if let Err(e) = scan::initial_scan(client.clone(), Path::new(&path), sem.clone()).await {
        tracing::error!(error = %e, "error during initial scan");
    }

    let mut watcher = Watcher::new(PathBuf::from(path));

    while let Some(path) = watcher.recv().await {
        let client = client.clone();
        let sem = sem.clone();
        tokio::spawn(async move {
            if let Err(e) = entry::handle_entry(client, path, sem).await {
                tracing::error!(error = %e, "error processing file");
            }
        });
    }

    Ok(())
}
