use std::fmt::{self, Display, Formatter};
use std::os::raw::c_char;

use num_enum::TryFromPrimitive;

/// A side of the market, either bid or ask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Ask,
    Bid,
}

impl From<Side> for char {
    fn from(side: Side) -> Self {
        match side {
            Side::Ask => 'A',
            Side::Bid => 'B',
        }
    }
}

impl From<Side> for c_char {
    fn from(side: Side) -> Self {
        char::from(side) as c_char
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Side {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_char(char::from(*self))
    }
}

/// A tick action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Modify,
    Trade,
    Cancel,
    Add,
    Status,
    Update,
}

impl From<Action> for char {
    fn from(action: Action) -> Self {
        match action {
            Action::Modify => 'M',
            Action::Trade => 'T',
            Action::Cancel => 'C',
            Action::Add => 'A',
            Action::Status => 'S',
            Action::Update => 'U',
        }
    }
}

impl From<Action> for c_char {
    fn from(action: Action) -> Self {
        char::from(action) as c_char
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Action {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_char(char::from(*self))
    }
}

/// A symbology type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(u8)]
pub enum SType {
    ProductId = 0,
    Native = 1,
    Smart = 2,
}

impl SType {
    /// Convert the symbology type to its `str` representation.
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SType::Native => "native",
            SType::Smart => "smart",
            SType::ProductId => "product_id",
        }
    }
}

impl Display for SType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A data record schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum Schema {
    /// Market by order.
    Mbo = 0,
    /// Market by price with a book depth of 1.
    Mbp1 = 1,
    /// Market by price with a book depth of 10.
    Mbp10 = 2,
    /// Combination of [Self::Trades] and [Self::Mbp1].
    Tbbo = 3,
    /// All trade events.
    Trades = 4,
    /// Open, high, low, close, and volume at a 1-second cadence.
    Ohlcv1S = 5,
    /// Open, high, low, close, and volume at a 1-minute cadence.
    Ohlcv1M = 6,
    /// Open, high, low, close, and volume at an hourly cadence.
    Ohlcv1H = 7,
    /// Open, high, low, close, and volume at a daily cadence.
    Ohlcv1D = 8,
    /// Symbol definitions.
    Definition = 9,
    ///
    Statistics = 10,
    /// Exchange status.
    Status = 11,
}

impl Schema {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Schema::Mbo => "mbo",
            Schema::Mbp1 => "mbp-1",
            Schema::Mbp10 => "mbp-10",
            Schema::Tbbo => "tbbo",
            Schema::Trades => "trades",
            Schema::Ohlcv1S => "ohlcv-1s",
            Schema::Ohlcv1M => "ohlcv-1m",
            Schema::Ohlcv1H => "ohlcv-1h",
            Schema::Ohlcv1D => "ohlcv-1d",
            Schema::Definition => "definition",
            Schema::Statistics => "statistics",
            Schema::Status => "status",
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Schema {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A data encoding format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(u8)]
pub enum Encoding {
    /// Databento Binary Encoding + Zstandard compression.
    Dbz = 0,
    /// Comma-separated values.
    Csv = 1,
    /// JavaScript object notation.
    Json = 2,
}

impl Encoding {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Encoding::Dbz => "dbz",
            Encoding::Csv => "csv",
            Encoding::Json => "json",
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A compression format or none if is uncompressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(u8)]
pub enum Compression {
    /// Uncompressed.
    None = 0,
    /// zstd compression.
    ZStd = 1,
}
