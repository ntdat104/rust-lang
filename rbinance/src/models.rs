use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvgPrice {
    pub mins: i32,
    pub price: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub order_id: i64,
    pub price: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub maker_commission: i32,
    pub taker_commission: i32,
    pub buyer_commission: i32,
    pub seller_commission: i32,
    pub can_trade: bool,
}
