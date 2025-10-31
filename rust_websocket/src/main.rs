use websocket::ClientBuilder;
use websocket::OwnedMessage;
use serde::Deserialize;
use serde_json;

// ---- Structs ----------------------------------------------------

#[derive(Debug, Deserialize)]
struct WSMessage {
    stream: String,
    data: AggTradeData,
}

#[derive(Debug, Deserialize)]
struct AggTradeData {
    #[serde(rename = "e")]
    event_type: String,

    #[serde(rename = "E")]
    event_time: u64,

    #[serde(rename = "s")]
    symbol: String,

    #[serde(rename = "a")]
    agg_trade_id: u64,

    #[serde(rename = "p", deserialize_with = "str_to_f64")]
    price: f64,

    #[serde(rename = "q", deserialize_with = "str_to_f64")]
    quantity: f64,

    #[serde(rename = "f")]
    first_trade_id: u64,

    #[serde(rename = "l")]
    last_trade_id: u64,

    #[serde(rename = "T")]
    trade_time: u64,

    #[serde(rename = "m")]
    is_buyer_maker: bool,

    #[serde(rename = "M")]
    ignore: bool,
}

// Convert JSON string fields ("1.234") → f64
fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

// ---- Pretty printer for trade ----------------------------------
fn print_trade(stream: String, t: &AggTradeData) {
    println!("");
    println!(
        "{:<18} {:<10} {:<14} {:<12} {:<12} {:<10} {:<12} {:<12} {:<12} {:<14} {:<8} {:<5}",
        "Stream", "Event", "EventTime", "Symbol", "AggTradeId", "Price", "Quantity",
        "FirstID", "LastID", "TradeTime", "Maker", "M"
    );
    println!(
        "{:<18} {:<10} {:<14} {:<12} {:<12} {:<10} {:<12} {:<12} {:<12} {:<14} {:<8} {:<5}",
        stream,
        t.event_type,
        t.event_time,
        t.symbol,
        t.agg_trade_id,
        t.price,
        t.quantity,
        t.first_trade_id,
        t.last_trade_id,
        t.trade_time,
        t.is_buyer_maker,
        t.ignore
    );
}


// ---- Main --------------------------------------------------------

fn main() {
    let url = "wss://stream.binance.com/stream";
    println!("Connecting to {url} ...");

    let mut client = ClientBuilder::new(url)
        .unwrap()
        .connect_secure(None)
        .expect("Failed to connect");

    println!("Connected!");

    // Subscription
    let subscribe_msg = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": [
            "btcusdt@aggTrade",
            "ethusdt@aggTrade",
            "bnbusdt@aggTrade"
        ],
        "id": 1
    })
    .to_string();

    client
        .send_message(&OwnedMessage::Text(subscribe_msg))
        .expect("Failed to send subscription");

    println!("Subscription sent!");

    // ---- Read Messages Forever ---------------------------------
    loop {
        match client.recv_message() {
            Ok(OwnedMessage::Text(txt)) => {
                match serde_json::from_str::<WSMessage>(&txt) {
                    Ok(msg) => {
                        let trade = msg.data;

                        // print_trade(msg.stream, &trade);

                        // Filter big trades
                        if trade.quantity > 10.0 {
                            print_trade(msg.stream, &trade);
                        }

                    }
                    Err(err) => {
                        println!("Parse error: {err}");
                    }
                }
            }

            Ok(OwnedMessage::Ping(p)) => {
                client.send_message(&OwnedMessage::Pong(p)).ok();
            }

            Ok(OwnedMessage::Close(_)) => {
                println!("Server closed connection");
                break;
            }

            Ok(_) => {}

            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }
}


// version 2
// use serde::Deserialize;
// use serde_json;
// use futures::StreamExt;

// // -------------------- Helpers ---------------------

// pub fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     s.parse::<f64>().map_err(serde::de::Error::custom)
// }

// // -------------------- Models -----------------------

// #[derive(Debug, Deserialize)]
// pub struct AggTradeData {
//     #[serde(rename = "e")]
//     pub event_type: String,

//     #[serde(rename = "E")]
//     pub event_time: u64,

//     #[serde(rename = "s")]
//     pub symbol: String,

//     #[serde(rename = "a")]
//     pub agg_trade_id: u64,

//     #[serde(rename = "p", deserialize_with = "str_to_f64")]
//     pub price: f64,

//     #[serde(rename = "q", deserialize_with = "str_to_f64")]
//     pub quantity: f64,

//     #[serde(rename = "f")]
//     pub first_trade_id: u64,

//     #[serde(rename = "l")]
//     pub last_trade_id: u64,

//     #[serde(rename = "T")]
//     pub trade_time: u64,

//     #[serde(rename = "m")]
//     pub is_buyer_maker: bool,

//     #[serde(rename = "M")]
//     pub ignore: bool,
// }

// #[derive(Debug, Deserialize)]
// #[serde(tag = "stream", content = "data")]
// pub enum StreamMessage {
//     #[serde(rename = "btcusdt@aggTrade")]
//     BtcAgg(AggTradeData),

//     #[serde(rename = "ethusdt@aggTrade")]
//     EthAgg(AggTradeData),

//     #[serde(rename = "bnbusdt@aggTrade")]
//     BnbAgg(AggTradeData),

//     #[serde(other)]
//     Unknown,
// }

// // -------------------- Main ------------------------
// use futures::{SinkExt};
// use tokio::sync::mpsc;
// use tokio_tungstenite::connect_async;
// use tokio_tungstenite::tungstenite::Message;

// #[tokio::main]
// async fn main() {
//     const URL: &str = "wss://stream.binance.com/stream";

//     println!("Connecting to {URL}...");
//     let (ws_stream, _) = connect_async(URL).await.expect("WebSocket failed");
//     println!("Connected!");

//     let (mut write, mut read) = ws_stream.split();

//     let (tx, mut rx) = mpsc::channel::<StreamMessage>(500);

//     // Subscription message
//     let subscribe_msg = serde_json::json!({
//         "method": "SUBSCRIBE",
//         "params": [
//             "btcusdt@aggTrade",
//             "ethusdt@aggTrade",
//             "bnbusdt@aggTrade"
//         ],
//         "id": 1
//     });

//     write
//         .send(Message::text(subscribe_msg.to_string()))
//         .await
//         .expect("Subscription failed");

//     println!("Subscription sent!");

//     // -------------------------------
//     // Task 1: WebSocket listener
//     // -------------------------------
//     let tx_clone = tx.clone();
//     tokio::spawn(async move {
//         while let Some(msg) = read.next().await {
//             if let Ok(Message::Text(text)) = msg {
//                 match serde_json::from_str::<StreamMessage>(&text) {
//                     Ok(parsed) => {
//                         if tx_clone.send(parsed).await.is_err() {
//                             println!("Receiver dropped");
//                             break;
//                         }
//                     }
//                     Err(e) => {
//                         println!("Parse error: {}", e);
//                     }
//                 }
//             }
//         }
//     });

//     // -------------------------------
//     // Task 2: Worker thread
//     // -------------------------------
//     tokio::spawn(async move {
//         while let Some(msg) = rx.recv().await {
//             match msg {
//                 StreamMessage::BtcAgg(t)
//                 | StreamMessage::EthAgg(t)
//                 | StreamMessage::BnbAgg(t) => {
//                     if t.quantity > 1.0 {
//                         println!(
//                             "[BIG TRADE] {} price={} qty={}",
//                             t.symbol, t.price, t.quantity
//                         );
//                     }
//                 }
//                 StreamMessage::Unknown => {}
//             }
//         }
//     });

//     tokio::signal::ctrl_c().await.unwrap();
//     println!("Shutdown!");
// }

