use super::indexer_client::IndexerClient;
use super::http_error::HttpError;
use std::collections::HashMap;
use serde_json::Value;

pub struct Account {}
impl Account {
    pub async fn get_subaccounts(
        &self, 
        indexer_client: &IndexerClient,
        address: &str, 
        limit: Option<i32>
    ) -> Result<Value, HttpError> {
        let endpoint = format!("/v4/addresses/{}", address);
        let mut params = HashMap::new();
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        indexer_client.get(&endpoint, Some(params)).await
    }
    pub async fn get_subaccount(
        &self, 
        indexer_client: &IndexerClient,
        address: &str, 
        subaccount_number: i32
    ) -> Result<Value, HttpError> {
        let endpoint = format!("/v4/addresses/{}/subaccountNumber/{}", address, subaccount_number);
        indexer_client.get(&endpoint, None).await
    }

    pub async fn get_subaccount_perpetual_positions(
        &self, 
        indexer_client: &IndexerClient,
        address: &str, 
        subaccount_number: i32, 
        status: Option<&str>, 
        limit: Option<i32>, 
        created_before_or_at_height: Option<i32>, 
        created_before_or_at_time: Option<&str>
    ) -> Result<Value, HttpError> {
        let endpoint = "/v4/perpetualPositions";
        let mut params = HashMap::new();
        params.insert("address", address.to_string());
        params.insert("subaccountNumber", subaccount_number.to_string());
        if let Some(status) = status {
            params.insert("status", status.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(created_before_or_at_height) = created_before_or_at_height {
            params.insert("createdBeforeOrAtHeight", created_before_or_at_height.to_string());
        }
        if let Some(created_before_or_at_time) = created_before_or_at_time {
            params.insert("createdBeforeOrAt", created_before_or_at_time.to_string());
        }
        indexer_client.get(endpoint, Some(params)).await
    }

    pub async fn get_subaccount_asset_transfers(
        &self,
        indexer_client: &IndexerClient, 
        address: &str, 
        subaccount_number: i32, 
        ticker: Option<&str>, 
        ticker_type: Option<&str>, 
        limit: Option<i32>, 
        created_before_or_at_height: Option<i32>, 
        created_before_or_at_time: Option<&str>
    ) -> Result<Value, HttpError> {
        let endpoint = "/v4/fills";
        let mut params = HashMap::new();
        params.insert("address", address.to_string());
        params.insert("subaccountNumber", subaccount_number.to_string());
        if let Some(ticker) = ticker {
            params.insert("market", ticker.to_string());
        }
        if let Some(ticker_type) = ticker_type {
            params.insert("marketType", ticker_type.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(created_before_or_at_height) = created_before_or_at_height {
            params.insert("createdBeforeOrAtHeight", created_before_or_at_height.to_string());
        }
        if let Some(created_before_or_at_time) = created_before_or_at_time {
            params.insert("createdBeforeOrAt", created_before_or_at_time.to_string());
        }
        indexer_client.get(endpoint, Some(params)).await
    }

    pub async fn get_subaccount_fills(
        &self, 
        indexer_client: &IndexerClient,
        address: &str, 
        subaccount_number: i32, 
        ticker: Option<&str>, 
        ticker_type: Option<&str>, 
        limit: Option<i32>, 
        created_before_or_at_height: Option<i32>, 
        created_before_or_at_time: Option<&str>
    ) -> Result<Value, HttpError> {
        let endpoint = "/v4/fills";
        let mut params = HashMap::new();
        params.insert("address", address.to_string());
        params.insert("subaccountNumber", subaccount_number.to_string());
        if let Some(ticker) = ticker {
            params.insert("market", ticker.to_string());
        }
        if let Some(ticker_type) = ticker_type {
            params.insert("marketType", ticker_type.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(created_before_or_at_height) = created_before_or_at_height {
            params.insert("createdBeforeOrAtHeight", created_before_or_at_height.to_string());
        }
        if let Some(created_before_or_at_time) = created_before_or_at_time {
            params.insert("createdBeforeOrAt", created_before_or_at_time.to_string());
        }
        indexer_client.get(endpoint, Some(params)).await
    }

    pub async fn get_subaccount_historical_pnls(
        &self, 
        indexer_client: &IndexerClient,
        address: &str, 
        subaccount_number: i32, 
        effective_before_or_at: Option<&str>, 
        effective_at_or_after: Option<&str>
    ) -> Result<Value, HttpError> {
        let endpoint = "/v4/historical-pnl";
        let mut params = HashMap::new();
        params.insert("address", address.to_string());
        params.insert("subaccountNumber", subaccount_number.to_string());
        if let Some(effective_before_or_at) = effective_before_or_at {
            params.insert("effectiveBeforeOrAt", effective_before_or_at.to_string());
        }
        if let Some(effective_at_or_after) = effective_at_or_after {
            params.insert("effectiveAtOrAfter", effective_at_or_after.to_string());
        }
        indexer_client.get(endpoint, Some(params)).await
    }
}
