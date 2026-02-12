use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackLyrics {
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

impl TrackLyrics {
    pub fn lyrics(&self) -> &str {
        self.synced_lyrics
            .as_deref()
            .unwrap_or(self.plain_lyrics.as_deref().unwrap_or(""))
    }
}
