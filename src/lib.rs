//! # gpui-symbols
//!
//! SF Symbols rendering for GPUI applications on macOS.
//!
//! This crate provides two levels of API:
//!
//! 1. **`SfSymbol`** - Low-level builder for rendering SF Symbols to raw RGBA pixels
//!    or GPUI `RenderImage`.
//!
//! 2. **`Icon`** (requires `component` feature) - High-level GPUI component that can
//!    be used directly in views, similar to GPUI Components.
//!
//! ## Features
//!
//! - `gpui` - Enable GPUI integration (converts to `RenderImage`)
//! - `component` - Enable high-level `Icon` component (implies `gpui`)
//!
//! ## Example: Using SfSymbol
//!
//! ```rust,ignore
//! use gpui_symbols::SfSymbol;
//!
//! // Render to raw RGBA pixels
//! let (width, height, data) = SfSymbol::new("star.fill")
//!     .size(32.0)
//!     .color(0x000000)
//!     .render_rgba()
//!     .unwrap();
//!
//! // With GPUI feature enabled:
//! let image = SfSymbol::new("star.fill").render().unwrap();
//! ```
//!
//! ## Example: Using Icon Component
//!
//! ```rust,ignore
//! use gpui_symbols::{Icon, define_icons};
//! use gpui::px;
//!
//! // Define your own icon enum
//! define_icons! {
//!     pub enum AppIcon {
//!         Star => "star.fill",
//!         Heart => "heart.fill",
//!         Settings => "gearshape.fill",
//!     }
//! }
//!
//! // Use in a view
//! fn view(window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!     div()
//!         .child(Icon::new("star.fill"))
//!         .child(Icon::from_name(AppIcon::Heart).text_color(0xFF0000))
//! }
//! ```

mod platform;
mod symbol;

#[cfg(feature = "component")]
mod icon;

// Re-export main types
pub use symbol::SfSymbol;

#[cfg(feature = "component")]
pub use icon::{Icon, IconName};

// Re-export sfsymbols presets
#[cfg(feature = "presets")]
pub use sfsymbols;
