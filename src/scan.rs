use crate::{client::LrcLib, entry};
use async_walkdir::WalkDir;
use futures::StreamExt;
use std::{path::Path, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet};

pub async fn initial_scan(client: LrcLib, path: &Path) -> color_eyre::Result<()> {
    let mut entries = WalkDir::new(path);
    let sem = Arc::new(Semaphore::new(50));
    let mut set = JoinSet::new();

    while let Some(entry_res) = entries.next().await {
        match entry_res {
            Ok(entry) => {
                let client = client.clone();
                let sem = sem.clone();
                set.spawn(async move {
                    let permit = sem.acquire_owned().await.unwrap();
                    let _permit = permit;
                    let path = entry.path();
                    if let Err(e) = entry::handle_entry(client, entry.path()).await {
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
