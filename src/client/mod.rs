use const_format::formatcp;
use serde::Serialize;

use crate::client::{file::SongInfo, track_lyrics::TrackLyrics};

pub mod file;
pub mod track_lyrics;

#[derive(Debug, Clone)]
pub struct LrcLib {
    client: reqwest::Client,
}

const BASE_URL: &'static str = "https://lrclib.net/api";
const GET: &'static str = formatcp!("{BASE_URL}/get");
// const GET_CACHED: &'static str = formatcp!("{BASE_URL}/get-cached");

impl LrcLib {
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("lrcnab 0.1.0 (private, contact notnullptr on discord for concerns!)")
            .build()?;

        Ok(Self { client })
    }

    async fn get_inner(
        &self,
        info: &SongInfo,
        url: &str,
    ) -> Result<Option<TrackLyrics>, reqwest::Error> {
        #[derive(Serialize)]
        struct Query<'a> {
            track_name: &'a str,
            artist_name: &'a str,
            album_name: &'a str,
            duration: u32,
        }

        let resp = self
            .client
            .get(url)
            .query(&Query {
                track_name: &info.track_name,
                artist_name: &info.artist_name,
                album_name: &info.album_name,
                duration: info.duration.as_secs() as u32,
            })
            .send()
            .await?;

        if resp.status().is_success() {
            let lyrics = resp.json::<TrackLyrics>().await?;
            Ok(Some(lyrics))
        } else {
            Ok(None)
        }
    }

    // async fn get_uncached(&self, info: &SongInfo) -> Result<Option<TrackLyrics>, reqwest::Error> {
    //     self.get_inner(info, GET).await
    // }

    // async fn get_cached(&self, info: &SongInfo) -> Result<Option<TrackLyrics>, reqwest::Error> {
    //     self.get_inner(info, GET_CACHED).await
    // }

    pub async fn get(&self, info: &SongInfo) -> Result<Option<TrackLyrics>, reqwest::Error> {
        // their cache seems to be messed up ?
        self.get_inner(info, GET).await
    }
}
