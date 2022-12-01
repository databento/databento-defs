//! A crate containing common Databento enums and data structs.
#[deny(missing_docs)] // document or use #[doc(hidden)]
#[deny(rustdoc::broken_intra_doc_links)]
#[deny(clippy::missing_errors_doc)]
#[forbid(unsafe_code)]
pub mod enums;
pub mod error;
pub mod record;

pub use error::{Error, Result};
