//! SF Symbols presets - type-safe enums for all SF Symbols versions.
//!
//! # Features
//!
//! - **Major versions** (`v1` - `v7`): Merged enums containing all symbols from that major release
//! - **Minor versions** (`v1-0`, `v6-4`, etc.): Precise enums for specific point releases
//!
//! ```toml
//! # Latest major version (default)
//! sfsymbols = "0.1"
//!
//! # Specific major version
//! sfsymbols = { version = "0.1", features = ["v6"] }
//!
//! # Specific minor versions
//! sfsymbols = { version = "0.1", default-features = false, features = ["v6-0", "v6-4"] }
//!
//! # All versions
//! sfsymbols = { version = "0.1", features = ["all"] }
//! ```

#[cfg(feature = "v1-0")]
mod v1_0;
#[cfg(feature = "v1-1")]
mod v1_1;
#[cfg(feature = "v2-0")]
mod v2_0;
#[cfg(feature = "v2-1")]
mod v2_1;
#[cfg(feature = "v2-2")]
mod v2_2;
#[cfg(feature = "v3-0")]
mod v3_0;
#[cfg(feature = "v3-1")]
mod v3_1;
#[cfg(feature = "v3-2")]
mod v3_2;
#[cfg(feature = "v3-3")]
mod v3_3;
#[cfg(feature = "v4-0")]
mod v4_0;
#[cfg(feature = "v4-1")]
mod v4_1;
#[cfg(feature = "v4-2")]
mod v4_2;
#[cfg(feature = "v5-0")]
mod v5_0;
#[cfg(feature = "v5-1")]
mod v5_1;
#[cfg(feature = "v5-2")]
mod v5_2;
#[cfg(feature = "v5-3")]
mod v5_3;
#[cfg(feature = "v5-4")]
mod v5_4;
#[cfg(feature = "v6-0")]
mod v6_0;
#[cfg(feature = "v6-1")]
mod v6_1;
#[cfg(feature = "v6-2")]
mod v6_2;
#[cfg(feature = "v6-3")]
mod v6_3;
#[cfg(feature = "v6-4")]
mod v6_4;
#[cfg(feature = "v7-0")]
mod v7_0;
#[cfg(feature = "v7-1")]
mod v7_1;

#[cfg(feature = "v1")]
mod v1;
#[cfg(feature = "v2")]
mod v2;
#[cfg(feature = "v3")]
mod v3;
#[cfg(feature = "v4")]
mod v4;
#[cfg(feature = "v5")]
mod v5;
#[cfg(feature = "v6")]
mod v6;
#[cfg(feature = "v7")]
mod v7;

#[cfg(feature = "v1-0")]
pub use v1_0::SfSymbolV1_0;
#[cfg(feature = "v1-1")]
pub use v1_1::SfSymbolV1_1;
#[cfg(feature = "v2-0")]
pub use v2_0::SfSymbolV2_0;
#[cfg(feature = "v2-1")]
pub use v2_1::SfSymbolV2_1;
#[cfg(feature = "v2-2")]
pub use v2_2::SfSymbolV2_2;
#[cfg(feature = "v3-0")]
pub use v3_0::SfSymbolV3_0;
#[cfg(feature = "v3-1")]
pub use v3_1::SfSymbolV3_1;
#[cfg(feature = "v3-2")]
pub use v3_2::SfSymbolV3_2;
#[cfg(feature = "v3-3")]
pub use v3_3::SfSymbolV3_3;
#[cfg(feature = "v4-0")]
pub use v4_0::SfSymbolV4_0;
#[cfg(feature = "v4-1")]
pub use v4_1::SfSymbolV4_1;
#[cfg(feature = "v4-2")]
pub use v4_2::SfSymbolV4_2;
#[cfg(feature = "v5-0")]
pub use v5_0::SfSymbolV5_0;
#[cfg(feature = "v5-1")]
pub use v5_1::SfSymbolV5_1;
#[cfg(feature = "v5-2")]
pub use v5_2::SfSymbolV5_2;
#[cfg(feature = "v5-3")]
pub use v5_3::SfSymbolV5_3;
#[cfg(feature = "v5-4")]
pub use v5_4::SfSymbolV5_4;
#[cfg(feature = "v6-0")]
pub use v6_0::SfSymbolV6_0;
#[cfg(feature = "v6-1")]
pub use v6_1::SfSymbolV6_1;
#[cfg(feature = "v6-2")]
pub use v6_2::SfSymbolV6_2;
#[cfg(feature = "v6-3")]
pub use v6_3::SfSymbolV6_3;
#[cfg(feature = "v6-4")]
pub use v6_4::SfSymbolV6_4;
#[cfg(feature = "v7-0")]
pub use v7_0::SfSymbolV7_0;
#[cfg(feature = "v7-1")]
pub use v7_1::SfSymbolV7_1;

#[cfg(feature = "v1")]
pub use v1::SfSymbolV1;
#[cfg(feature = "v2")]
pub use v2::SfSymbolV2;
#[cfg(feature = "v3")]
pub use v3::SfSymbolV3;
#[cfg(feature = "v4")]
pub use v4::SfSymbolV4;
#[cfg(feature = "v5")]
pub use v5::SfSymbolV5;
#[cfg(feature = "v6")]
pub use v6::SfSymbolV6;
#[cfg(feature = "v7")]
pub use v7::SfSymbolV7;

#[cfg(feature = "unified")]
mod unified;
/// Unified SF Symbols enum containing all symbols from all versions.
#[cfg(feature = "unified")]
pub use unified::SfSymbol;

/// Alias for the latest SF Symbols version (SfSymbolV7)
#[cfg(all(feature = "v7", not(feature = "unified")))]
pub type SfSymbol = SfSymbolV7;
