// src/clients/indexer_client.rs
use super::http_error::HttpError;
use reqwest::{Client, Response};

pub struct IndexerClient {
    base_url: String,
    client: Client,
}

impl IndexerClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub fn markets(&self) -> &Markets {
        &Markets
    }

    pub async fn get(&self, endpoint: &str, params: Option<HashMap<&str, String>>) -> Result<Response, HttpError> {
        let mut url = Url::parse(&format!("{}{}", self.base_url, endpoint)).map_err(|_| HttpError::UrlParseError)?;

        if let Some(params) = params {
            for (key, value) in params {
                url.query_pairs_mut().append_pair(key, &value);
            }
        }

        let response = self.client.get(url).send().await?;
        Ok(response)
    }
}