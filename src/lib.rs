mod candle;
pub mod traits;
mod utils;

pub use candle::Candle;

/// Represents the internally used floating type for the crate.
#[cfg(not(feature = "f32"))]
pub type Num = f64;

/// Represents the internally used floating type for the crate.
#[cfg(feature = "f32")]
pub type Num = f32;

/// Represents the internally used integer type for the crate.
#[cfg(not(feature = "i64"))]
pub type Time = u64;

/// Represents the internally used integer type for the crate.
#[cfg(feature = "i64")]
pub type Time = i64;
