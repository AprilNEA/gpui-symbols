//! GPUI Icon component for SF Symbols.
//!
//! This module provides a high-level Icon API similar to GPUI Components,
//! allowing SF Symbols to be used directly as GPUI elements.

use std::sync::Arc;

use gpui::{
    img, px, rgb, App, Hsla, ImageSource, IntoElement, Pixels, Rgba, RenderImage, SharedString,
    Styled, Window,
};

use crate::{RenderingMode, SfSymbol, SymbolScale, SymbolWeight};

/// Trait for types that can provide an SF Symbol name.
///
/// Implement this trait for your own enums to use them with [`Icon`].
///
/// # Example
///
/// ```rust,ignore
/// use gpui_symbols::{IconName, Icon};
///
/// #[derive(Clone, Copy)]
/// enum MyIcon {
///     Star,
///     Heart,
///     Gear,
/// }
///
/// impl IconName for MyIcon {
///     fn name(&self) -> &'static str {
///         match self {
///             MyIcon::Star => "star.fill",
///             MyIcon::Heart => "heart.fill",
///             MyIcon::Gear => "gearshape.fill",
///         }
///     }
/// }
///
/// // Now you can use it like GPUI Components:
/// let icon = Icon::from_name(MyIcon::Star).text_color(0xFF0000);
/// ```
pub trait IconName: Clone + 'static {
    /// Returns the SF Symbol name for this icon.
    fn name(&self) -> &'static str;
}

/// Implement IconName for &'static str for convenience.
impl IconName for &'static str {
    fn name(&self) -> &'static str {
        self
    }
}

// Implement IconName for sfsymbols enums
#[cfg(feature = "presets")]
macro_rules! impl_icon_name_for_sfsymbols {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IconName for $ty {
                fn name(&self) -> &'static str {
                    <$ty>::name(self)
                }
            }
        )*
    };
}

#[cfg(feature = "presets")]
impl_icon_name_for_sfsymbols!(
    sfsymbols::SfSymbolV1,
    sfsymbols::SfSymbolV2,
    sfsymbols::SfSymbolV3,
    sfsymbols::SfSymbolV4,
    sfsymbols::SfSymbolV5,
    sfsymbols::SfSymbolV6,
    sfsymbols::SfSymbolV7,
    sfsymbols::SfSymbolAll,
);

/// A GPUI-compatible Icon component for rendering SF Symbols.
///
/// This struct implements `IntoElement` and can be used directly in GPUI views,
/// similar to how icons work in GPUI Components.
///
/// # Example
///
/// ```rust,ignore
/// use gpui_symbols::Icon;
/// use gpui::px;
///
/// fn view(window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
///     div()
///         .child(Icon::new("star.fill"))
///         .child(Icon::new("heart.fill").text_color(0xFF0000).size(px(24.)))
/// }
/// ```
#[derive(Clone, IntoElement)]
pub struct Icon {
    name: SharedString,
    size: Pixels,
    color: Hsla,
    weight: SymbolWeight,
    symbol_scale: SymbolScale,
    rendering_mode: RenderingMode,
}

impl Icon {
    /// Create a new Icon with the given SF Symbol name.
    ///
    /// Default size is 16px, default color is black.
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            size: px(16.),
            color: gpui::black(),
            weight: SymbolWeight::default(),
            symbol_scale: SymbolScale::default(),
            rendering_mode: RenderingMode::default(),
        }
    }

    /// Create a new Icon from a type implementing [`IconName`].
    pub fn from_name<T: IconName>(icon: T) -> Self {
        Self::new(icon.name())
    }

    /// Set the size of the icon in pixels.
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// Alias for [`Icon::size`] for backward compatibility.
    #[inline]
    pub fn with_size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// Set the color of the icon using any type that converts to [`Hsla`].
    ///
    /// Accepts GPUI color types: `Hsla`, `Rgba`, or use helper functions like
    /// `gpui::rgb(0xFF0000)`, `gpui::hsla(h, s, l, a)`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui::{rgb, hsla};
    ///
    /// Icon::new("star.fill").color(rgb(0xFF0000));
    /// Icon::new("heart.fill").color(hsla(0.0, 1.0, 0.5, 1.0));
    /// ```
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    /// Set the color of the icon as an RGB hex value.
    ///
    /// This is a convenience method. For more color options, use [`Icon::color`].
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// Icon::new("star.fill").text_color(0xFF0000) // Red
    /// ```
    pub fn text_color(mut self, hex: u32) -> Self {
        self.color = rgb(hex).into();
        self
    }

    /// Set the symbol weight (default: Regular).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui_symbols::{Icon, SymbolWeight};
    ///
    /// Icon::new("star.fill").weight(SymbolWeight::Bold);
    /// ```
    pub fn weight(mut self, weight: SymbolWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set the symbol scale (default: Medium).
    ///
    /// Controls the overall visual weight of the symbol independent of point size.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui_symbols::{Icon, SymbolScale};
    ///
    /// Icon::new("star.fill").symbol_scale(SymbolScale::Large);
    /// ```
    pub fn symbol_scale(mut self, scale: SymbolScale) -> Self {
        self.symbol_scale = scale;
        self
    }

    /// Set the rendering mode (default: Hierarchical).
    ///
    /// - `Monochrome`: Single color for entire symbol
    /// - `Hierarchical`: Primary color with automatic opacity layers
    /// - `Palette`: Multiple distinct colors
    /// - `Multicolor`: Original multicolor design
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui_symbols::{Icon, RenderingMode};
    ///
    /// Icon::new("cloud.sun.fill").rendering_mode(RenderingMode::Multicolor);
    /// ```
    pub fn rendering_mode(mut self, mode: RenderingMode) -> Self {
        self.rendering_mode = mode;
        self
    }

    /// Render the icon to an image.
    fn render_image(&self) -> Option<Arc<RenderImage>> {
        let size: f32 = self.size.into();
        // Convert Hsla -> Rgba -> u32 (RGB only, ignore alpha for now)
        let rgba: Rgba = self.color.into();
        let color_u32: u32 = rgba.into();
        // Shift from RGBA to RGB format (drop alpha byte)
        let rgb = color_u32 >> 8;

        SfSymbol::new(self.name.as_ref())
            .size(size)
            .color(rgb)
            .weight(self.weight)
            .symbol_scale(self.symbol_scale)
            .rendering_mode(self.rendering_mode)
            .render()
    }
}

impl gpui::RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.size;

        if let Some(image) = self.render_image() {
            img(ImageSource::Render(image)).size(size).into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}

/// Convenience macro for creating Icon enums with SF Symbol mappings.
///
/// # Example
///
/// ```rust,ignore
/// use gpui_symbols::define_icons;
///
/// define_icons! {
///     pub enum AppIcon {
///         Star => "star.fill",
///         Heart => "heart.fill",
///         Settings => "gearshape.fill",
///     }
/// }
///
/// // Use it:
/// let icon = Icon::from_name(AppIcon::Star);
/// ```
#[macro_export]
macro_rules! define_icons {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $symbol:literal),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis enum $name {
            $($variant),*
        }

        impl $crate::IconName for $name {
            fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $symbol),*
                }
            }
        }
    };
}
