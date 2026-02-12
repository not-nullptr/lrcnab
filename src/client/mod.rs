pub mod file;

pub struct LrcLib {
    client: reqwest::Client,
}

impl LrcLib {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
