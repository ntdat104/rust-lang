use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub status: u16,
    pub message: String,
    pub data: StockData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockData {
    pub price_close: f64,
    pub price_open: f64,
    pub price_low: f64,
    pub price_high: f64,
    pub price_floor: f64,
    pub price_ceiling: f64,
    pub price_reference: f64,
    pub price_average: f64,
    pub total_value: f64,
    pub buy_quantity: f64,
    pub sell_quantity: f64,
    pub net_change: f64,
    pub pct_change: f64,
    pub total_volume: f64,
    pub total_buy_volume: f64,
    pub total_sell_volume: f64,
    pub price_bid1: f64,
    pub quantity_bid1: f64,
    pub price_bid2: f64,
    pub quantity_bid2: f64,
    pub price_bid3: f64,
    pub quantity_bid3: f64,
    pub price_ask1: f64,
    pub quantity_ask1: f64,
    pub price_ask2: f64,
    pub quantity_ask2: f64,
    pub price_ask3: f64,
    pub quantity_ask3: f64,
    pub buy_foreign_quantity: f64,
    pub sell_foreign_quantity: f64,
    pub buy_foreign_value: f64,
    pub sell_foreign_value: f64,
    // #[serde(rename = "type")]
    pub r#type: String, // "type" is reserved in Rust
}
