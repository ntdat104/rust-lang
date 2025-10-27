mod domain;
use std::collections::HashMap;
use domain::{ ApiResponse };
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reqwest_client = reqwest::Client
        ::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        "en",
        "vi",
        "Hello, world"
    );

    println!("Fetching URL: {}", url);

    let text_translate = reqwest_client.get(url).send().await?.text().await?;

    println!("{text_translate:#?}");

    let resp2 = reqwest_client
        .get("https://api2.simplize.vn/api/historical/quote/HPG")
        .send().await?
        .json::<HashMap<String, Value>>().await?;

    println!("Resp2: {:?}", resp2);

    let resp = reqwest_client
        .get("https://api2.simplize.vn/api/historical/quote/HPG")
        .send().await?
        .json::<ApiResponse>().await?;

    println!("Status: {}", resp.status);
    println!("Message: {}", resp.message);
    println!("Price Close: {}", resp.data.price_close);
    println!("Price Open: {}", resp.data.price_open);
    println!("Price Low: {}", resp.data.price_low);
    println!("Price High: {}", resp.data.price_high);
    println!("Price Floor: {}", resp.data.price_floor);
    println!("Price Ceiling: {}", resp.data.price_ceiling);
    println!("Price Reference: {}", resp.data.price_reference);
    println!("Price Average: {}", resp.data.price_average);
    println!("Total Value: {}", resp.data.total_value);
    println!("Buy Quantity: {}", resp.data.buy_quantity);
    println!("Sell Quantity: {}", resp.data.sell_quantity);
    println!("Net Change: {}", resp.data.net_change);
    println!("Pct Change: {}", resp.data.pct_change);
    println!("Total Volume: {}", resp.data.total_volume);
    println!("Total Buy Volume: {}", resp.data.total_buy_volume);
    println!("Total Sell Volume: {}", resp.data.total_sell_volume);
    println!("Price Bid1: {}", resp.data.price_bid1);
    println!("Quantity Bid1: {}", resp.data.quantity_bid1);
    println!("Price Bid2: {}", resp.data.price_bid2);
    println!("Quantity Bid2: {}", resp.data.quantity_bid2);
    println!("Price Bid3: {}", resp.data.price_bid3);
    println!("Quantity Bid3: {}", resp.data.quantity_bid3);
    println!("Price Ask1: {}", resp.data.price_ask1);
    println!("Quantity Ask1: {}", resp.data.quantity_ask1);
    println!("Price Ask2: {}", resp.data.price_ask2);
    println!("Quantity Ask2: {}", resp.data.quantity_ask2);
    println!("Price Ask3: {}", resp.data.price_ask3);
    println!("Quantity Ask3: {}", resp.data.quantity_ask3);
    println!("Buy Foreign Quantity: {}", resp.data.buy_foreign_quantity);
    println!("Sell Foreign Quantity: {}", resp.data.sell_foreign_quantity);
    println!("Buy Foreign Value: {}", resp.data.buy_foreign_value);
    println!("Sell Foreign Value: {}", resp.data.sell_foreign_value);
    println!("Type: {}", resp.data.r#type);

    Ok(())
}
