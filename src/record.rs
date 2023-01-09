//! Market data types for encoding different Databento [`Schema`](crate::enums::Schema)s and conversion functions.
use std::{mem, ops::RangeInclusive, os::raw::c_char, ptr::NonNull};

use crate::enums::SecurityUpdateAction;

/// Common data for all Databento records.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RecordHeader {
    /// The length of the message in 32-bit words.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub length: u8,
    /// The record type; with `0x00..0x0F` specifying booklevel size. Record
    /// types implement the trait [`ConstTypeId`], which contains a constant
    /// ID specific to that record type.
    pub rtype: u8,
    /// The publisher ID assigned by Databento.
    pub publisher_id: u16,
    /// The product ID assigned by the venue.
    pub product_id: u32,
    /// The matching engine received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_event: u64,
}

pub const TICK_MSG_TYPE_ID: u8 = 0xA0;
/// Market-by-order (MBO) tick message.
/// `hd.rtype = 0xA0`
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MboMsg {
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
    pub flags: u8,
    /// A channel ID within the venue.
    pub channel_id: u8,
    /// The event action. Can be M\[odify\], T\[rade\], C\[ancel\], A\[dd\]
    /// or special: \[S\]tatus, \[U\]pdate.
    pub action: c_char,
    /// The order side. Can be A\[sk\], B\[id\] or N\[one\].
    pub side: c_char,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
}

// Named `DB_BA` in C
/// A book level.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
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
    pub flags: u8,
    /// The depth of actual book change.
    pub depth: u8,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
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
    pub flags: u8,
    /// The depth of actual book change.
    pub depth: u8,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
    pub booklevel: [BidAskPair; 1],
}

/// Market by price implementation with a known book depth of 10.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
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
    pub flags: u8,
    /// The depth of actual book change.
    pub depth: u8,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_recv: u64,
    /// The delta of `ts_recv - ts_exchange_send`, max 2 seconds.
    pub ts_in_delta: i32,
    /// The message sequence number assigned at the venue.
    pub sequence: u32,
    pub booklevel: [BidAskPair; 10],
}

pub type TbboMsg = Mbp1Msg;

pub const OHLCV_TYPE_ID: u8 = 0x11;
/// Open, high, low, close, and volume.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
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
/// `hd.rtype = 0x12`
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StatusMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_recv: u64,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub group: [c_char; 21],
    pub trading_status: u8,
    pub halt_reason: u8,
    pub trading_event: u8,
}

pub const INSTRUMENT_DEF_MSG_TYPE_ID: u8 = 0x13;
/// Definition of an instrument.
/// `hd.rtype = 0x13`
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[doc(hidden)]
pub struct InstrumentDefMsg {
    /// The common header.
    pub hd: RecordHeader,
    /// The capture server received timestamp expressed as number of nanoseconds since UNIX epoch.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_recv: u64,
    pub min_price_increment: i64,
    pub display_factor: i64,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub expiration: u64,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
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
    pub maturity_year: u16,
    pub decay_start_date: u16,
    pub channel_id: u16,
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
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serialize_enum_as_char_repr")
    )]
    pub security_update_action: SecurityUpdateAction,
    pub maturity_month: u8,
    pub maturity_day: u8,
    pub maturity_week: u8,
    pub user_defined_instrument: c_char,
    pub contract_multiplier_unit: i8,
    pub flow_schedule_type: i8,
    pub tick_rule: u8,
    /// Adjust filler for alignment.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _dummy: [c_char; 3],
}

pub const IMBALANCE_TYPE_ID: u8 = 0x14;
/// Order imbalance message.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[doc(hidden)]
pub struct Imbalance {
    pub hd: RecordHeader,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
    pub ts_recv: u64,
    pub ref_price: i64,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_large_u64"))]
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

pub const GATEWAY_ERROR_MSG_TYPE_ID: u8 = 0x15;
/// Gateway error message
/// `hd.rtype = 0x15`
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GatewayErrorMsg {
    pub hd: RecordHeader,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub err: [c_char; 64],
}

pub const SYMBOL_MAPPING_MSG_TYPE_ID: u8 = 0x16;
/// Symbol mapping message
/// `hd.rtype = 0x16`
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "trivial_copy", derive(Copy))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SymbolMappingMsg {
    pub hd: RecordHeader,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub stype_in_symbol: [c_char; 22],
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_c_char_arr"))]
    pub stype_out_symbol: [c_char; 22],
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _dummy: [c_char; 4],
    pub start_ts: u64,
    pub end_ts: u64,
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

/// Serialize as a string to avoid any loss of precision with JSON serializers and parsers.
#[cfg(feature = "serde")]
fn serialize_large_u64<S: serde::Serializer>(num: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&num.to_string())
}

/// Serialize an enum as its char representation.
#[cfg(feature = "serde")]
fn serialize_enum_as_char_repr<S: serde::Serializer, T: Copy + Into<u8>>(
    val: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_char(Into::<u8>::into(*val) as char)
}

/// A trait for objects with polymorphism based around [`RecordHeader::rtype`].
pub trait ConstTypeId {
    /// The value of [`RecordHeader::rtype`] for the implementing type.
    const TYPE_ID: u8;
}

/// Provides a _relatively safe_ method for converting a reference to a
/// struct beginning with the header into a [`RecordHeader`].
/// Because it accepts a reference, the lifetime of the returned reference
/// is tied to the input.
///
/// # Safety
/// Although this function accepts a reference to a [`ConstTypeId`], it's assumed this struct's
/// binary representation begins with a RecordHeader value
pub unsafe fn transmute_into_header<T: ConstTypeId>(record: &T) -> &RecordHeader {
    // Safety: because it comes from a reference, `header` must not be null. It's ok to cast to `mut`
    // because it's never mutated.
    let non_null = NonNull::from(record);
    non_null.cast::<RecordHeader>().as_ref()
}

/// Provides a _relatively safe_ method for converting a reference to
/// [`RecordHeader`] to a struct beginning with the header. Because it accepts a
/// reference, the lifetime of the returned reference is tied to the input. This
/// function checks `rtype` before casting to ensure `bytes` contains a `T`.
///
/// # Safety
/// `raw` must contain at least `std::mem::size_of::<T>()` bytes and a valid
/// [`RecordHeader`] instance.
pub unsafe fn transmute_record_bytes<T: ConstTypeId>(bytes: &[u8]) -> Option<&T> {
    assert!(
        bytes.len() >= mem::size_of::<T>(),
        concat!(
            "Passing a slice smaller than `",
            stringify!(T),
            "` to `transmute_record_bytes` is invalid"
        )
    );
    let non_null = NonNull::new_unchecked(bytes.as_ptr() as *mut u8);
    if non_null.cast::<RecordHeader>().as_ref().rtype == T::TYPE_ID {
        Some(non_null.cast::<T>().as_ref())
    } else {
        None
    }
}

/// Provides a _relatively safe_ method for converting a view on bytes into a
/// a [`RecordHeader`].
/// Because it accepts a reference, the lifetime of the returned reference is
/// tied to the input.
///
/// # Safety
/// `bytes` must contain a complete record (not only the header). This is so that
/// the header can be subsequently passed to transmute_record
pub unsafe fn transmute_header_bytes(bytes: &[u8]) -> Option<&RecordHeader> {
    assert!(
        bytes.len() >= mem::size_of::<RecordHeader>(),
        concat!(
            "Passing a slice smaller than `",
            stringify!(RecordHeader),
            "` to `transmute_header_bytes` is invalid"
        )
    );
    let non_null = NonNull::new_unchecked(bytes.as_ptr() as *mut u8);
    let header = non_null.cast::<RecordHeader>().as_ref();
    if header.length as usize * 4 > bytes.len() {
        None
    } else {
        Some(header)
    }
}

/// Provides a _relatively safe_ method for converting a reference to a
/// [`RecordHeader`] to a struct beginning with the header. Because it accepts a reference,
/// the lifetime of the returned reference is tied to the input.
///
/// # Safety
/// Although this function accepts a reference to a [`RecordHeader`], it's assumed this is
/// part of a larger `T` struct.
pub unsafe fn transmute_record<T: ConstTypeId>(header: &RecordHeader) -> Option<&T> {
    if header.rtype == T::TYPE_ID {
        // Safety: because it comes from a reference, `header` must not be null. It's ok to cast to `mut`
        // because it's never mutated.
        let non_null = NonNull::from(header);
        Some(non_null.cast::<T>().as_ref())
    } else {
        None
    }
}

/// Provides a _relatively safe_ method for converting a mut reference to a
/// [`RecordHeader`] to a struct beginning with the header. Because it accepts a reference,
/// the lifetime of the returned reference is tied to the input.
///
/// # Safety
/// Although this function accepts a reference to a [`RecordHeader`], it's assumed this is
/// part of a larger `T` struct.
pub unsafe fn transmute_record_mut<T: ConstTypeId>(header: &mut RecordHeader) -> Option<&mut T> {
    if header.rtype == T::TYPE_ID {
        // Safety: because it comes from a reference, `header` must not be null. It's ok to cast to `mut`
        // because it's never mutated.
        let non_null = NonNull::from(header);
        Some(non_null.cast::<T>().as_mut())
    } else {
        None
    }
}

impl ConstTypeId for MboMsg {
    const TYPE_ID: u8 = TICK_MSG_TYPE_ID;
}

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

impl ConstTypeId for OhlcvMsg {
    const TYPE_ID: u8 = OHLCV_TYPE_ID;
}

impl ConstTypeId for StatusMsg {
    const TYPE_ID: u8 = STATUS_MSG_TYPE_ID;
}

impl ConstTypeId for InstrumentDefMsg {
    const TYPE_ID: u8 = INSTRUMENT_DEF_MSG_TYPE_ID;
}

impl ConstTypeId for Imbalance {
    const TYPE_ID: u8 = IMBALANCE_TYPE_ID;
}

impl ConstTypeId for GatewayErrorMsg {
    const TYPE_ID: u8 = GATEWAY_ERROR_MSG_TYPE_ID;
}

impl ConstTypeId for SymbolMappingMsg {
    const TYPE_ID: u8 = SYMBOL_MAPPING_MSG_TYPE_ID;
}

#[cfg(test)]
mod tests {
    use super::*;

    const OHLCV_MSG: OhlcvMsg = OhlcvMsg {
        hd: RecordHeader {
            length: 56,
            rtype: 17,
            publisher_id: 1,
            product_id: 5482,
            ts_event: 1609160400000000000,
        },
        open: 372025000000000,
        high: 372050000000000,
        low: 372025000000000,
        close: 372050000000000,
        volume: 57,
    };

    #[test]
    fn test_transmute_record_bytes() {
        unsafe {
            let ohlcv_bytes = std::slice::from_raw_parts(
                &OHLCV_MSG as *const OhlcvMsg as *const u8,
                mem::size_of::<OhlcvMsg>(),
            )
            .to_vec();
            let ohlcv = transmute_record_bytes::<OhlcvMsg>(ohlcv_bytes.as_slice()).unwrap();
            assert_eq!(*ohlcv, OHLCV_MSG);
        };
    }

    #[test]
    #[should_panic]
    fn test_transmute_record_bytes_small_buffer() {
        let source = OHLCV_MSG;
        unsafe {
            let slice = std::slice::from_raw_parts(
                &source as *const OhlcvMsg as *const u8,
                mem::size_of::<OhlcvMsg>() - 5,
            );
            transmute_record_bytes::<OhlcvMsg>(slice);
        };
    }

    #[test]
    fn test_transmute_record() {
        let source = Box::new(OHLCV_MSG);
        let ohlcv_ref: &OhlcvMsg = unsafe { transmute_record(&source.hd) }.unwrap();
        assert_eq!(*ohlcv_ref, OHLCV_MSG);
    }

    #[test]
    fn test_transmute_record_mut() {
        let mut source = Box::new(OHLCV_MSG);
        let ohlcv_ref: &OhlcvMsg = unsafe { transmute_record_mut(&mut source.hd) }.unwrap();
        assert_eq!(*ohlcv_ref, OHLCV_MSG);
    }

    #[test]
    fn test_symbol_mapping_size() {
        assert_eq!(mem::size_of::<SymbolMappingMsg>(), 80);
    }
}
