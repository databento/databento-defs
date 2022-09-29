use std::{ops::RangeInclusive, os::raw::c_char};

use crate::Error;

/// Common data for all Databento Records, i.e. types implementing the trait
/// [`TryFrom<Record>`].
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RecordHeader {
    /// The length of the message in 32-bit words.
    pub length: u8,
    /// The record type; with `0x00..0x0F` specifying booklevel size.
    pub rtype: u8,
    /// The publisher ID assigned by Databento.
    pub publisher_id: u16,
    /// The product ID assigned by the venue.
    pub product_id: u32,
    /// The matching engine received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_event: u64,
}

pub const TICK_MSG_TYPE_ID: u8 = 0xA0;
/// Market-by-order (MBO tick message.
/// `hd.type_ = 0xA0`
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TickMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The order ID assigned at the venue.
    pub order_id: u64,
    /// The order price expressed as a signed integer where every 1 unit
    /// corresponds to 1e-9, i.e. 1/1,000,000,000 or 0.000000001.
    pub price: i64,
    /// The order quantity.
    pub size: u32,
    /// A combination of packet end with matching engine status.
    pub flags: i8,
    /// A channel ID within the venue.
    pub channel_id: u8,
    /// The event action. Can be M\[odify\], T\[rade\], C\[ancel\], A\[dd\]
    /// or special: \[S\]tatus, \[U\]pdate.
    pub action: c_char,
    /// The order side. Can be A\[sk\], B\[id\] or N\[one\].
    pub side: c_char,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
}

// Named `DB_BA` in C
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BidAskPair {
    /// The bid price.
    pub bid_px: i64,
    /// The ask price.
    pub ask_px: i64,
    /// The bid size.
    pub bid_sz: u32,
    /// The ask size.
    pub ask_sz: u32,
    /// The bid order count.
    pub bid_ct: u32,
    /// The ask order count.
    pub ask_ct: u32,
}

pub const MAX_UA_BOOK_LEVEL: usize = 0xF;
pub const MBP_MSG_TYPE_ID_RANGE: RangeInclusive<u8> = 0x00..=(MAX_UA_BOOK_LEVEL as u8);

/// Market by price implementation with a book depth of 0. Equivalent to
/// MBP-0.
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TradeMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The order price expressed as a signed integer where every 1 unit
    /// corresponds to 1e-9, i.e. 1/1,000,000,000 or 0.000000001.
    pub price: i64,
    /// The order quantity.
    pub size: u32,
    /// The event action. Can be M\[odify\], T\[rade\], C\[ancel\], A\[dd\]
    /// or special: \[S\]tatus, \[U\]pdate.
    pub action: c_char,
    /// The order side. Can be A\[sk\], B\[id\] or N\[one\].
    pub side: c_char,
    /// A combination of packet end with matching engine status.
    pub flags: i8,
    /// The depth of actual book change.
    pub depth: u8,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub booklevel: [BidAskPair; 0],
}

/// Market by price implementation with a known book depth of 1.
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Mbp1Msg {
    /// The common header.
    pub hd: RecordHeader,
    /// The order price expressed as a signed integer where every 1 unit
    /// corresponds to 1e-9, i.e. 1/1,000,000,000 or 0.000000001.
    pub price: i64,
    /// The order quantity.
    pub size: u32,
    /// The event action. Can be M\[odify\], T\[rade\], C\[ancel\], A\[dd\]
    /// or special: \[S\]tatus, \[U\]pdate.
    pub action: c_char,
    /// The order side. Can be A\[sk\], B\[id\] or N\[one\].
    pub side: c_char,
    /// A combination of packet end with matching engine status.
    pub flags: i8,
    /// The depth of actual book change.
    pub depth: u8,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
    pub booklevel: [BidAskPair; 1],
}

/// Market by price implementation with a known book depth of 10.
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Mbp10Msg {
    /// The common header.
    pub hd: RecordHeader,
    /// The order price expressed as a signed integer where every 1 unit
    /// corresponds to 1e-9, i.e. 1/1,000,000,000 or 0.000000001.
    pub price: i64,
    /// The order quantity.
    pub size: u32,
    /// The event action. Can be M\[odify\], T\[rade\], C\[ancel\], A\[dd\]
    /// or special: \[S\]tatus, \[U\]pdate.
    pub action: c_char,
    /// The order side. Can be A\[sk\], B\[id\] or N\[one\].
    pub side: c_char,
    /// A combination of packet end with matching engine status.
    pub flags: i8,
    /// The depth of actual book change.
    pub depth: u8,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
    pub booklevel: [BidAskPair; 10],
}

pub type TbboMsg = Mbp1Msg;

pub const OHLCV_TYPE_ID: u8 = 0x11;
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OhlcvMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The open price for the bar.
    pub open: i64,
    /// The high price for the bar.
    pub high: i64,
    /// The low price for the bar.
    pub low: i64,
    /// The close price for the bar.
    pub close: i64,
    /// The total volume traded during the aggregation period.
    pub volume: u64,
}

pub const STATUS_MSG_TYPE_ID: u8 = 0x12;
/// Trading status update message
/// `hd.type_ = 0x12`
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StatusMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_recv: u64,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub group: [c_char; 21],
    pub trading_status: u8,
    pub halt_reason: u8,
    pub trading_event: u8,
}

pub const SYM_DEF_MSG_TYPE_ID: u8 = 0x13;
// Named `SymdefMsg` in C
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SymDefMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    pub ts_recv: u64,
    pub min_price_increment: i64,
    pub display_factor: i64,
    pub expiration: u64,
    pub activation: u64,
    pub high_limit_price: i64,
    pub low_limit_price: i64,
    pub max_price_variation: i64,
    pub trading_reference_price: i64,
    pub unit_of_measure_qty: i64,
    pub min_price_increment_amount: i64,
    pub price_ratio: i64,
    pub inst_attrib_value: i32,
    pub underlying_id: u32,
    pub cleared_volume: i32,
    pub market_depth_implied: i32,
    pub market_depth: i32,
    pub market_segment_id: u32,
    pub max_trade_vol: u32,
    pub min_lot_size: i32,
    pub min_lot_size_block: i32,
    pub min_lot_size_round_lot: i32,
    pub min_trade_vol: u32,
    pub open_interest_qty: i32,
    pub contract_multiplier: i32,
    pub decay_quantity: i32,
    pub original_contract_size: i32,
    pub related_security_id: u32,
    pub trading_reference_date: u16,
    pub appl_id: i16,
    pub maturity_month_year: u16,
    pub decay_start_date: u16,
    pub chan: u16,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub currency: [c_char; 4],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub settl_currency: [c_char; 4],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub secsubtype: [c_char; 6],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub symbol: [c_char; 22],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub group: [c_char; 21],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub exchange: [c_char; 5],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub asset: [c_char; 7],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub cfi: [c_char; 7],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub security_type: [c_char; 7],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub unit_of_measure: [c_char; 31],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub underlying: [c_char; 21],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub related: [c_char; 21],
    pub match_algorithm: c_char,
    pub md_security_trading_status: u8,
    pub main_fraction: u8,
    pub price_display_format: u8,
    pub settl_price_type: u8,
    pub sub_fraction: u8,
    pub underlying_product: u8,
    pub security_update_action: c_char,
    pub maturity_month_month: u8,
    pub maturity_month_day: u8,
    pub maturity_month_week: u8,
    pub user_defined_instrument: c_char,
    pub contract_multiplier_unit: i8,
    pub flow_schedule_type: i8,
    pub tick_rule: u8,
    /// Adjust filler for alignment.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _dummy: [c_char; 3],
}

/// Order imbalance message.
pub const IMBALANCE_TYPE_ID: u8 = 0x14;
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Imbalance {
    pub hd: RecordHeader,
    pub ts_recv: u64,
    pub ref_price: i64,
    pub auction_time: u64,
    /// Continuous book clearing price.
    pub cont_book_clr_price: i64,
    /// Auction interest clearing price.
    pub auct_interest_clr_price: i64,
    // Short-selling restriction filling price.
    pub ssr_filling_price: i64,
    /// Indicative match price.
    pub ind_match_price: i64,
    pub upper_collar: i64,
    pub lower_collar: i64,
    pub paired_qty: u32,
    pub total_imbalance_qty: u32,
    pub market_imbalance_qty: u32,
    pub auction_type: c_char,
    pub side: c_char,
    pub auction_status: u8,
    pub freeze_status: u8,
    pub num_extensions: u8,
    pub unpaired_qty: u8,
    pub unpaired_side: c_char,
    pub significant_imbalance: c_char,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _dummy: [c_char; 4],
}

#[cfg(feature = "serde")]
fn serialize_c_char_arr<S: serde::Serializer, const N: usize>(
    arr: &[c_char; N],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let cstr = unsafe { std::ffi::CStr::from_ptr(&arr[0]) };
    let str = cstr.to_str().unwrap_or("<invalid UTF-8>");
    serializer.serialize_str(str)
}

/// A polymorphic record of a particular schema provided by Databento for which the
/// trait [`TryFrom<Record>`] is implemented. Using [`TryFrom<Record>`] is the primary
/// way of interacting with this struct.
#[derive(Debug)]
pub struct Record {
    /// Opaque non-owned pointer to some record struct with a [`RecordHeader`].
    pub ptr: *const RecordHeader,
}

impl Record {
    pub fn new(ptr: *const RecordHeader) -> crate::Result<Self> {
        if ptr.is_null() {
            Err(crate::Error::NullPointer)
        } else {
            Ok(Self { ptr })
        }
    }
}

/// A trait for objects with polymorphism based around [`RecordHeader.rtype`].
pub trait ConstTypeId {
    const TYPE_ID: u8;
}

impl ConstTypeId for TickMsg {
    const TYPE_ID: u8 = TICK_MSG_TYPE_ID;
}

/// Macro for implementing [`TryFrom<Record>`] for the given type. The Rust orphan rules for trait
/// implementations prevent blanket implementing [`TryFrom<Record>`] for all types implementing
/// [ConstTypeId](https://doc.rust-lang.org/stable/error-index.html#E0210).
/// Using a macro also improves the specificity of the error message for [Error::TypeConversion].
#[macro_export]
macro_rules! try_from_record {
    ($tick_type:ident) => {
        impl TryFrom<Record> for $tick_type {
            type Error = Error;

            fn try_from(record: Record) -> $crate::Result<Self> {
                // Safety: null pointer checked in `new`
                unsafe {
                    if record.ptr.read().rtype == Self::TYPE_ID {
                        Ok(record.ptr.cast::<Self>().read())
                    } else {
                        Err(Error::TypeConversion(concat!(
                            "Not a ",
                            stringify!($tick_type)
                        )))
                    }
                }
            }
        }
    };
}

try_from_record!(TickMsg);

/// [TradeMsg]'s type ID is the size of its `booklevel` array (0) and is
/// equivalent to MBP-0.
impl ConstTypeId for TradeMsg {
    const TYPE_ID: u8 = 0;
}

/// [Mbp1Msg]'s type ID is the size of its `booklevel` array.
impl ConstTypeId for Mbp1Msg {
    const TYPE_ID: u8 = 1;
}

/// [Mbp10Msg]'s type ID is the size of its `booklevel` array.
impl ConstTypeId for Mbp10Msg {
    const TYPE_ID: u8 = 10;
}

try_from_record!(TradeMsg);
try_from_record!(Mbp1Msg);
try_from_record!(Mbp10Msg);

impl ConstTypeId for OhlcvMsg {
    const TYPE_ID: u8 = OHLCV_TYPE_ID;
}

try_from_record!(OhlcvMsg);

impl ConstTypeId for StatusMsg {
    const TYPE_ID: u8 = STATUS_MSG_TYPE_ID;
}

try_from_record!(StatusMsg);

impl ConstTypeId for SymDefMsg {
    const TYPE_ID: u8 = SYM_DEF_MSG_TYPE_ID;
}

try_from_record!(SymDefMsg);

impl ConstTypeId for Imbalance {
    const TYPE_ID: u8 = IMBALANCE_TYPE_ID;
}

try_from_record!(Imbalance);
