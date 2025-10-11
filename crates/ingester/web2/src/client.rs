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
            .user_agent("CryptoPortfolio/1.0")
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
