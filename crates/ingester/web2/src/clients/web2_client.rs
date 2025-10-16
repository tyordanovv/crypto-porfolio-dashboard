use reqwest::Client;
use std::time::Duration;

pub struct Web2Client {
    http: Client,
    fred_api_key: String,
}

impl Web2Client {
    pub fn new(fred_api_key: String) -> Self {

        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to create HTTP client");

        Self { http, fred_api_key }
    }

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn fred_api_key(&self) -> &str {
        &self.fred_api_key
    }
}
