use lofty::{
    file::{AudioFile as _, TaggedFileExt},
    tag::ItemKey,
};
use std::{path::PathBuf, time::Duration};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioFileError {
    #[error("lofty error: {0}")]
    Lofty(#[from] lofty::error::LoftyError),

    #[error("necessary tags not found in file")]
    MissingTags,
}

pub struct SongInfo {
    pub artist_name: String,
    pub album_name: String,
    pub track_name: String,
    pub duration: Duration,
}

impl SongInfo {
    pub async fn read<P: Into<PathBuf>>(path: P) -> Result<Self, AudioFileError> {
        let path = path.into();
        let info = tokio::task::spawn_blocking(move || {
            lofty::read_from_path(&path).map(|metadata| {
                let Some(tag) = metadata.primary_tag() else {
                    return Err(AudioFileError::MissingTags);
                };

                let artist_name = tag
                    .get_string(ItemKey::TrackArtist)
                    .ok_or(AudioFileError::MissingTags)?
                    .to_string();

                let album_name = tag
                    .get_string(ItemKey::AlbumTitle)
                    .ok_or(AudioFileError::MissingTags)?
                    .to_string();

                let track_name = tag
                    .get_string(ItemKey::TrackTitle)
                    .ok_or(AudioFileError::MissingTags)?
                    .to_string();

                let duration = metadata.properties().duration();

                Ok(Self {
                    artist_name,
                    album_name,
                    track_name,
                    duration,
                })
            })
        })
        .await
        .unwrap()??;

        Ok(info)
    }
}
