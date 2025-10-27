use std::time::{ SystemTime, UNIX_EPOCH };
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use hex::encode as hex_encode;
use reqwest::blocking::Client;
use serde_json::Value;

use crate::config::Config;

type HmacSha256 = Hmac<Sha256>;

pub struct BinanceClient {
    config: Config,
    http: Client,
}

impl BinanceClient {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            http: Client::new(),
        }
    }

    fn sign(&self, query: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.config.secret_key.as_bytes()).unwrap();
        mac.update(query.as_bytes());
        hex_encode(mac.finalize().into_bytes())
    }

    fn timestamp() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }

    /// GET /api/v3/avgPrice
    pub fn get_avg_price(&self, symbol: &str) -> Value {
        let url = format!("{}/api/v3/avgPrice?symbol={}", self.config.endpoint, symbol);
        self.http
            .get(&url)
            .header("X-MBX-APIKEY", &self.config.api_key)
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    /// GET /api/v3/account (requires signature)
    pub fn get_account(&self) -> Value {
        let timestamp = Self::timestamp();
        let query = format!("timestamp={}", timestamp);
        let signature = self.sign(&query);
        let url = format!(
            "{}/api/v3/account?{}&signature={}",
            self.config.endpoint,
            query,
            signature
        );

        self.http
            .get(&url)
            .header("X-MBX-APIKEY", &self.config.api_key)
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    /// GET /api/v3/allOrders
    pub fn get_all_orders(&self, symbol: &str) -> Value {
        let timestamp = Self::timestamp();
        let query = format!("symbol={}&timestamp={}", symbol, timestamp);
        let signature = self.sign(&query);
        let url = format!(
            "{}/api/v3/allOrders?{}&signature={}",
            self.config.endpoint,
            query,
            signature
        );

        self.http
            .get(&url)
            .header("X-MBX-APIKEY", &self.config.api_key)
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    /// POST /api/v3/order
    pub fn post_new_order(
        &self,
        symbol: &str,
        side: &str,
        order_type: &str,
        quantity: f64,
        price: Option<f64>,
        time_in_force: Option<&str>
    ) -> Value {
        let timestamp = Self::timestamp();

        let mut query = format!(
            "symbol={}&side={}&type={}&quantity={}&timestamp={}",
            symbol,
            side,
            order_type,
            quantity,
            timestamp
        );

        if order_type.eq_ignore_ascii_case("LIMIT") {
            let tif = time_in_force.unwrap_or("GTC");
            let p = price.expect("LIMIT order requires price");
            query = format!("{}&price={}&timeInForce={}", query, p, tif);
        }

        let signature = self.sign(&query);
        let url = format!(
            "{}/api/v3/order?{}&signature={}",
            self.config.endpoint,
            query,
            signature
        );

        self.http
            .post(&url)
            .header("X-MBX-APIKEY", &self.config.api_key)
            .send()
            .unwrap()
            .json()
            .unwrap()
    }
}
