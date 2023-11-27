use crate::clients::http_error::HttpError;
use crate::clients::account::Account;
use crate::clients::markets::Markets;
use crate::clients::utility::Utility;
use std::collections::HashMap;
use reqwest::{Client, Url};
use serde_json::Value;

pub struct IndexerClient {
    base_url: String,
    client: Client,
    pub account: Account,
    pub markets: Markets,
    pub utility: Utility,
}

impl IndexerClient {
    pub fn new(base_url: &str) -> Self {
        let indexer_client = Self {
            base_url: base_url.to_string(),
            client: Client::new(),
            account: Account{},
            markets: Markets{},
            utility: Utility{},
        };

        indexer_client
    }
    pub async fn get(
        &self, 
        endpoint: &str, 
        params: Option<HashMap<&str, String>>
    ) -> Result<Value, HttpError> {
        let mut url = Url::parse(&format!("{}{}", self.base_url, endpoint)).map_err(|_| HttpError::UrlParseError)?;
        if let Some(params) = params {
            for (key, value) in params {
                url.query_pairs_mut().append_pair(key, &value);
            }
        }
        let response = self.client.get(url).send().await.map_err(|e| HttpError::from(e))?;
        let body = response.json().await.map_err(|e| HttpError::from(e))?;
        Ok(body)
    }
}