use super::indexer_client::IndexerClient;
use super::http_error::HttpError;
use std::collections::HashMap;
use serde_json::Value;

pub struct Utility {}

impl Utility {
    pub async fn get_time(&self, indexer_client: &IndexerClient) -> Result<Value, HttpError> {
        indexer_client.get("/v4/time", None).await
    }

    pub async fn get_height(&self, indexer_client: &IndexerClient) -> Result<Value, HttpError> {
        indexer_client.get("/v4/height", None).await
    }

    // TODO: use more appropriate address type
    pub async fn screen(&self, indexer_client: &IndexerClient, address: &str) -> Result<Value, HttpError> {
        let mut params = HashMap::new();
        params.insert("address", address.to_string());
        indexer_client.get("/v4/screen", Some(params)).await
    }
}