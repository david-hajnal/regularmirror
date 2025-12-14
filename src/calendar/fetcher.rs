use reqwest::blocking::Client;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(HttpClient { client })
    }

    pub fn fetch_url(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client.get(url).send()?;

        if response.status().is_success() {
            let text = response.text()?;
            Ok(text)
        } else {
            Err(format!("HTTP error: {}", response.status()).into())
        }
    }
}
