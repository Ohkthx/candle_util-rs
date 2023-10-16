//! Demonstrates how to initialize and use the various traits.
use candle_util::traits::{Ohlc4, Timestamp};
use candle_util::Candle;

fn main() {
    // Test candle.
    let raw = r#"{
        "start": 5,
        "open": "5.5",
        "close": 7.5,
        "high": "8.5",
        "low": "4.5",
        "volume": "2.5"
    }"#;

    let candle: Candle = serde_json::from_str(&raw).unwrap();

    println!("Timestamp: {:#?}", candle.timestamp());
    println!("OHLC Value: {}", candle.ohlc4());
}
