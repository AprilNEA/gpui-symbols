//! # gpui-sfsymbols
//!
//! SF Symbols for GPUI applications.
//!
//! This crate re-exports [`gpui_symbols`] with the `presets` feature enabled,
//! providing access to all 9000+ SF Symbols as type-safe enums.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gpui_sfsymbols::{Icon, sfsymbols::SfSymbolV7};
//!
//! let icon = Icon::from_name(SfSymbolV7::StarFill);
//! ```

pub use gpui_symbols::*;
