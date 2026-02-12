use tokio::fs::File;

use crate::client::{LrcLib, file::SongInfo};
use std::path::PathBuf;

pub async fn handle_entry(client: LrcLib, path: PathBuf) -> color_eyre::Result<()> {
    let metadata = tokio::fs::metadata(&path).await?;

    if !metadata.is_file() {
        tracing::debug!(path = %path.display(), "skipping non-file entry");
        return Ok(());
    }

    let with_lrc = path.with_extension("lrc");
    if tokio::fs::metadata(&with_lrc).await.is_ok() {
        tracing::debug!(path = %path.display(), "skipping file with existing .lrc");
        return Ok(());
    }

    let is_audio = path.extension().map_or(false, |ext| {
        matches!(
            ext.to_str().unwrap_or_default().to_lowercase().as_str(),
            "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "opus"
        )
    });

    if !is_audio {
        tracing::debug!(path = %path.display(), "skipping non-audio file");
        return Ok(());
    }

    tracing::debug!(path = %path.display(), "processing file");

    let info = SongInfo::read(&path).await?;

    tracing::debug!(path = %path.display(), track_name = %info.track_name, artist_name = %info.artist_name, album_name = %info.album_name, duration = ?info.duration, "got file info");

    tracing::info!(name = %info.track_name, "searching for lyrics");

    let Some(track) = client.get(&info).await? else {
        tracing::info!(name = %info.track_name, "no lyrics found");
        // write out an empty lrc file to prevent future lookups
        File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(with_lrc)
            .await?;

        return Ok(());
    };

    tracing::info!(name = %info.track_name, "lyrics found, saving");

    tokio::fs::write(with_lrc, track.lyrics()).await?;

    tracing::info!(name = %info.track_name, "lyrics saved");

    Ok(())
}
