use reqwest::{header::HeaderValue, Url};

const BASE_URL: &str = "https://api.switch-bot.com";

pub struct SwitchBotClient {
    client: reqwest::Client,
    base_url: Url,
}

impl SwitchBotClient {
    pub async fn get_devises(&self) -> Result<String, Box<dyn std::error::Error>> {
        let path = "v1.0/devices";
        let endpoint = format!("{}{}", self.base_url, path);
        let res = self.client.get(endpoint).send().await?;
        let body = res.text().await?;
        Ok(body)
    }
}

pub struct ClientBuilder {
    pub api_key: String,
}

impl ClientBuilder {
    pub fn new(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            api_key: api_key.into(),
        }
    }

    pub fn build(&self) -> Result<SwitchBotClient, Box<dyn std::error::Error>> {
        let mut header_map = reqwest::header::HeaderMap::new();
        header_map.insert(
            "Authorization",
            HeaderValue::from_str(self.api_key.as_str()).unwrap(),
        );

        let client = reqwest::Client::builder()
            .user_agent("rust-switchbot")
            .default_headers(header_map) // Specify headers for each requests
            .build()?;

        Ok(SwitchBotClient {
            client: client,
            base_url: Url::parse(BASE_URL).unwrap(),
        })
    }
}
