use rbinance::{ BinanceClient, Config };

fn main() {
    let cfg = Config::from_file("config.toml");
    let client = BinanceClient::new(Config {
        api_key: cfg.api_key,
        secret_key: cfg.secret_key,
        endpoint: cfg.endpoint,
    });

    let avg_price = client.get_avg_price("BTCUSDT");
    println!("Average Price: {:?}", avg_price);

    let account = client.get_account();
    println!("Account Info: {:?}", account);

    // 1️⃣ Get all orders
    let orders = client.get_all_orders("BTCUSDT");
    println!("All Orders: {:#?}", orders);

    // 2️⃣ Place new limit order
    // let new_order = client.post_new_order(
    //     "BTCUSDT",
    //     "BUY",
    //     "LIMIT",
    //     0.001,
    //     Some(65000.0),
    //     Some("GTC") // ✅ mandatory for LIMIT
    // );
    // println!("New Order Response: {:#?}", new_order);
}
