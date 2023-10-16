use crate::utils::from_str;
use crate::{traits::*, Num, Time};
use candle_derive::{Close, High, Low, Open, Start, Timestamp, Volume};
use serde::{Deserialize as Deserial, Serialize as Serial};

/// Represents a candle for a product.
#[derive(Serial, Deserial, Debug, Clone, Timestamp, Start, Open, Close, Low, High, Volume)]
pub struct Candle {
    /// Timestamp for bucket start time, in UNIX time.
    #[serde(deserialize_with = "from_str")]
    pub start: Time,
    /// Lowest price during the bucket interval.
    #[serde(deserialize_with = "from_str")]
    pub low: Num,
    /// Highest price during the bucket interval.
    #[serde(deserialize_with = "from_str")]
    pub high: Num,
    /// Opening price (first trade) in the bucket interval.
    #[serde(deserialize_with = "from_str")]
    pub open: Num,
    /// Closing price (last trade) in the bucket interval.
    #[serde(deserialize_with = "from_str")]
    pub close: Num,
    /// Volume of trading activity during the bucket interval.
    #[serde(deserialize_with = "from_str")]
    pub volume: Num,
}

impl Hl2 for Candle {}
impl Hlc3 for Candle {}
impl Ohlc4 for Candle {}
