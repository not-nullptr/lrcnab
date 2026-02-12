use lofty::{file::TaggedFileExt, tag::ItemKey};
use std::path::PathBuf;
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
    pub file_path: String,
}

pub struct AudioFile {
    pub path: PathBuf,
    pub info: SongInfo,
}

impl AudioFile {
    pub async fn read<P: Into<PathBuf>>(path: P) -> Result<Self, AudioFileError> {
        let path = path.into();
        let (metadata, path) = tokio::task::spawn_blocking(move || {
            lofty::read_from_path(&path).map(|file| (file, path))
        })
        .await
        .unwrap()?;

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

        Ok(Self {
            path,
            info: SongInfo {
                artist_name,
                album_name,
                track_name,
                file_path: path.to_string_lossy().to_string(),
            },
        })
    }
}

async fn do_thing() {
    println!("doing thing");
}
