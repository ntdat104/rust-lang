use chrono::{NaiveDateTime, TimeZone, Utc, DateTime};

fn main() {
    // Example input time
    let input = "2025-03-10 08:24:55";

    // Parse using the source format
    let dt = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse datetime");

    // Format to DD/MM/YYYY hh:mm:ss
    let output = dt.format("%d/%m/%Y %H:%M:%S").to_string();

    println!("{}", output); // -> 10/03/2025 08:24:55

    // parsing with timestamp
    let timestamp = 1730608655i64;

    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
    let formatted = dt.format("%d/%m/%Y %H:%M:%S").to_string();

    println!("{}", formatted);

    // parsing with iso
    let iso = "2025-03-10T08:24:55Z";

    let dt: DateTime<Utc> = iso.parse().expect("Invalid ISO 8601");

    let formatted = dt.format("%d/%m/%Y %H:%M:%S").to_string();

    println!("{}", formatted);
}
