use super::IndexerClient;
use super::http_error::HttpError;
use reqwest::Response;

pub struct Markets;

impl Markets {
    pub async fn get_perpetual_markets(&self, client: &IndexerClient, markets: Vec<&str>) -> Result<Response, HttpError> {
        let endpoint = "/v4/perpetual-markets";
        let mut params = HashMap::new();
        params.insert("ticker", markets.join(","));
        client.get(endpoint, Some(params)).await
    }

    pub async fn get_perpetual_markets(&self, client: &IndexerClient, market: &str) -> Result<Response, HttpError> {
        let endpoint = format!("/v4/perpetual-markets/orderbooks/perpetualMarket/{}", market);
        client.get(endpoint, None).await
    }

    pub async fn get_perpetual_market_trades(
        &self, 
        client: &IndexerClient, 
        market: &str, 
        starting_before_or_at_height: Option<i32>, 
        limit: Option<i32>
    ) -> Result<Response, HttpError> {
        let mut endpoint = format!("/v4/trades/perpetualMarket/{}", market);
        let mut params = HashMap::new();
        if let Some(height) = starting_before_or_at_height {
            params.insert("createdBeforeOrAtHeight", height.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }

        client.get(&endpoint, params).await
    }

    pub async fn get_candles(
        &self, 
        client: &IndexerClient, 
        market: &str, 
        resolution: &str, 
        from_iso: Option<&str>, 
        to_iso: Option<&str>, 
        limit: Option<i32>
    ) -> Result<Response, HttpError> {
        let endpoint = format!("/v4/candles/perpetualMarkets/{}", market);
        let mut params = HashMap::new();
        params.insert("resolution", resolution.to_string());
        if let Some(from_iso) = from_iso {
            params.insert("fromISO", from_iso.to_string());
        }
        if let Some(to_iso) = to_iso {
            params.insert("toISO", to_iso.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        client.get(&endpoint, Some(params)).await
    }

    pub async fn get_perpetual_market_funding(
        &self, 
        client: &IndexerClient, 
        market: &str, 
        effective_before_or_at: Option<&str>, 
        effective_before_or_at_height: Option<i32>, 
        limit: Option<i32>
    ) -> Result<Response, HttpError> {
        let endpoint = format!("/v4/historicalFunding/{}", market);
        let mut params = HashMap::new();
        if let Some(effective_before_or_at) = effective_before_or_at {
            params.insert("effectiveBeforeOrAt", effective_before_or_at.to_string());
        }
        if let Some(effective_before_or_at_height) = effective_before_or_at_height {
            params.insert("effectiveBeforeOrAtHeight", effective_before_or_at_height.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        client.get(&endpoint, Some(params)).await
    }

    pub async fn get_perpetual_markets_sparklines(
        &self, 
        client: &IndexerClient, 
        period: &str
    ) -> Result<Response, HttpError> {
        let endpoint = "/v4/sparklines";
        let mut params = HashMap::new();
        params.insert("timePeriod", period.to_string());
        client.get(endpoint, Some(params)).await
    }
}