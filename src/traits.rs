use crate::{Num, Time};

/// User Defined: Timestamp for the data type.
pub trait Timestamp {
    /// Timestamp for the data type.
    fn timestamp(&self) -> Time;
}

/// User Defined: Starting timestamp for the data type.
pub trait Start {
    /// Starting timestamp for the data type.
    fn start(&self) -> Time;
}

/// User Defined: Opening value for the data type.
pub trait Open {
    /// Opening value for the data type.
    fn open(&self) -> Num;
}

/// User Defined: Closing value for the data type.
pub trait Close {
    /// Closing value for the data type.
    fn close(&self) -> Num;
}

/// User Defined: Highest value for the data type.
pub trait High {
    /// Highest value for the data type.
    fn high(&self) -> Num;
}

/// User Defined: Lowest value for the data type.
pub trait Low {
    /// Lowest value for the data type.
    fn low(&self) -> Num;
}

/// User Defined: Total volume for the data type.
pub trait Volume {
    /// Total volume for the data type.
    fn volume(&self) -> Num;
}

/// Average between High and Low traits.
pub trait Hl2: High + Low {
    /// Average between High and Low traits.
    fn hl2(&self) -> Num {
        (self.high() + self.low()) / 2.0 as Num
    }
}

/// Average between High, Low, and Close traits.
pub trait Hlc3: High + Low + Close {
    /// Average between High, Low, and Close traits.
    fn hlc3(&self) -> Num {
        (self.high() + self.low() + self.close()) / 3.0 as Num
    }
}

/// Average between Open, High, Low, and Close traits.
pub trait Ohlc4: Open + High + Low + Close {
    /// Average between Open, High, Low, and Close traits.
    fn ohlc4(&self) -> Num {
        (self.open() + self.high() + self.low() + self.close()) / 4.0 as Num
    }
}
