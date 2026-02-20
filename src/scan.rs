use crate::{client::LrcLib, entry};
use async_walkdir::WalkDir;
use futures::StreamExt;
use std::{path::Path, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet};

pub async fn initial_scan(
    client: LrcLib,
    path: &Path,
    sem: Arc<Semaphore>,
) -> color_eyre::Result<()> {
    let mut entries = WalkDir::new(path);
    let mut set = JoinSet::new();

    while let Some(entry_res) = entries.next().await {
        match entry_res {
            Ok(entry) => {
                let client = client.clone();
                let sem = sem.clone();
                set.spawn(async move {
                    let path = entry.path();
                    if let Err(e) = entry::handle_entry(client, entry.path(), sem).await {
                        tracing::error!(path = %path.display(), error = %e, "error processing file");
                    }
                });
            }
            Err(e) => tracing::error!(error = %e, "error reading directory entry"),
        }
    }

    while let Some(res) = set.join_next().await {
        if let Err(e) = res {
            eprintln!("task panicked: {e}");
        }
    }

    Ok(())
}
