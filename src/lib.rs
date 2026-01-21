//! # gpui-symbols
//!
//! SF Symbols rendering for GPUI applications on macOS.
//!
//! ## Features
//!
//! - `gpui` - Enable GPUI integration (converts to `RenderImage`)
//!
//! ## Example
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

#[cfg(feature = "gpui")]
use std::sync::Arc;

#[cfg(feature = "gpui")]
use gpui::RenderImage;
#[cfg(feature = "gpui")]
use image::{Frame, RgbaImage};
#[cfg(feature = "gpui")]
use smallvec::smallvec;

#[cfg(feature = "component")]
use gpui::{img, px, App, ImageSource, IntoElement, Pixels, SharedString, Styled, Window};

#[cfg(target_os = "macos")]
mod platform {
    use objc::runtime::{Class, Object};
    use objc::{msg_send, sel, sel_impl};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NSSize {
        pub width: f64,
        pub height: f64,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NSPoint {
        pub x: f64,
        pub y: f64,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NSRect {
        pub origin: NSPoint,
        pub size: NSSize,
    }

    /// Render SF Symbol to RGBA pixel buffer
    pub fn render_sf_symbol(
        name: &str,
        point_size: f32,
        scale: f32,
        color: u32,
    ) -> Option<(u32, u32, Vec<u8>)> {
        unsafe {
            // 1. Create NSString
            let name_cstr = std::ffi::CString::new(name).ok()?;
            let ns_string_class = Class::get("NSString")?;
            let ns_name: *mut Object =
                msg_send![ns_string_class, stringWithUTF8String: name_cstr.as_ptr()];
            if ns_name.is_null() {
                return None;
            }

            // 2. Create NSImage with system symbol name
            let ns_image_class = Class::get("NSImage")?;
            let symbol_image: *mut Object = msg_send![
                ns_image_class,
                imageWithSystemSymbolName: ns_name
                accessibilityDescription: std::ptr::null::<Object>()
            ];
            if symbol_image.is_null() {
                return None;
            }

            // 3. Create color from RGB
            let ns_color_class = Class::get("NSColor")?;
            let r = ((color >> 16) & 0xFF) as f64 / 255.0;
            let g = ((color >> 8) & 0xFF) as f64 / 255.0;
            let b = (color & 0xFF) as f64 / 255.0;
            let symbol_color: *mut Object = msg_send![
                ns_color_class,
                colorWithRed: r
                green: g
                blue: b
                alpha: 1.0f64
            ];

            // 4. Create symbol configuration with size and color
            let config_class = Class::get("NSImageSymbolConfiguration")?;

            let size_config: *mut Object = msg_send![
                config_class,
                configurationWithPointSize: point_size as f64
                weight: 0.0f64
                scale: 2i64  // NSImageSymbolScaleLarge
            ];

            let color_config: *mut Object =
                msg_send![config_class, configurationWithHierarchicalColor: symbol_color];

            let combined_config: *mut Object =
                msg_send![size_config, configurationByApplyingConfiguration: color_config];

            // 5. Apply configuration
            let configured_image: *mut Object =
                msg_send![symbol_image, imageWithSymbolConfiguration: combined_config];
            let symbol_image = if configured_image.is_null() {
                symbol_image
            } else {
                configured_image
            };

            // 6. Get symbol size
            let ns_size: NSSize = msg_send![symbol_image, size];
            let pixel_width = (ns_size.width * scale as f64).ceil() as u32;
            let pixel_height = (ns_size.height * scale as f64).ceil() as u32;

            if pixel_width == 0 || pixel_height == 0 {
                return None;
            }

            // 7. Create NSBitmapImageRep for drawing
            let bitmap_class = Class::get("NSBitmapImageRep")?;
            let bitmap_rep: *mut Object = msg_send![bitmap_class, alloc];
            let color_space_name: *mut Object = msg_send![
                ns_string_class,
                stringWithUTF8String: b"NSCalibratedRGBColorSpace\0".as_ptr()
            ];
            let bitmap_rep: *mut Object = msg_send![
                bitmap_rep,
                initWithBitmapDataPlanes: std::ptr::null::<*mut u8>()
                pixelsWide: pixel_width as isize
                pixelsHigh: pixel_height as isize
                bitsPerSample: 8isize
                samplesPerPixel: 4isize
                hasAlpha: true
                isPlanar: false
                colorSpaceName: color_space_name
                bitmapFormat: 0u64
                bytesPerRow: (pixel_width * 4) as isize
                bitsPerPixel: 32isize
            ];

            if bitmap_rep.is_null() {
                return None;
            }

            // 8. Create NSGraphicsContext and draw
            let ns_graphics_context_class = Class::get("NSGraphicsContext")?;
            let context: *mut Object =
                msg_send![ns_graphics_context_class, graphicsContextWithBitmapImageRep: bitmap_rep];
            if context.is_null() {
                return None;
            }

            let old_context: *mut Object = msg_send![ns_graphics_context_class, currentContext];
            let _: () = msg_send![ns_graphics_context_class, setCurrentContext: context];

            // 9. Fill white background
            let white_color: *mut Object = msg_send![ns_color_class, whiteColor];
            let _: () = msg_send![white_color, set];
            let fill_rect = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize {
                    width: pixel_width as f64,
                    height: pixel_height as f64,
                },
            };
            let ns_bezier_class = Class::get("NSBezierPath")?;
            let _: () = msg_send![ns_bezier_class, fillRect: fill_rect];

            // 10. Draw symbol
            let draw_rect = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize {
                    width: pixel_width as f64,
                    height: pixel_height as f64,
                },
            };
            let from_rect = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: ns_size,
            };
            let _: () = msg_send![
                symbol_image,
                drawInRect: draw_rect
                fromRect: from_rect
                operation: 2u64  // NSCompositingOperationSourceOver
                fraction: 1.0f64
            ];

            // 11. Restore context
            let _: () = msg_send![ns_graphics_context_class, setCurrentContext: old_context];

            // 12. Get pixel data
            let bitmap_data: *const u8 = msg_send![bitmap_rep, bitmapData];
            if bitmap_data.is_null() {
                return None;
            }

            let data_len = (pixel_width * pixel_height * 4) as usize;
            let mut buffer = vec![0u8; data_len];
            std::ptr::copy_nonoverlapping(bitmap_data, buffer.as_mut_ptr(), data_len);

            Some((pixel_width, pixel_height, buffer))
        }
    }
}

/// SF Symbol builder for rendering system symbols.
///
/// # Example
///
/// ```rust,ignore
/// use gpui_symbols::SfSymbol;
///
/// // Render to raw RGBA
/// let (width, height, data) = SfSymbol::new("star.fill")
///     .render_rgba()
///     .unwrap();
///
/// // With gpui feature:
/// let image = SfSymbol::new("heart.fill")
///     .size(48.0)
///     .color(0xFF0000)
///     .render()
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct SfSymbol {
    name: String,
    size: f32,
    scale: f32,
    color: u32,
}

impl SfSymbol {
    /// Create a new SF Symbol with the given name.
    ///
    /// See [SF Symbols](https://developer.apple.com/sf-symbols/) for available symbol names.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: 32.0,
            scale: 2.0,
            color: 0x000000, // black
        }
    }

    /// Set the point size of the symbol (default: 32.0).
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the scale factor for rendering (default: 2.0 for Retina).
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Set the color as RGB hex value (default: 0x000000 black).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// SfSymbol::new("star.fill")
    ///     .color(0xFF0000)  // red
    ///     .render();
    /// ```
    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    /// Render to raw RGBA pixel data.
    ///
    /// Returns `(width, height, rgba_data)` or `None` if rendering fails.
    #[cfg(target_os = "macos")]
    pub fn render_rgba(&self) -> Option<(u32, u32, Vec<u8>)> {
        platform::render_sf_symbol(&self.name, self.size, self.scale, self.color)
    }

    /// Render to raw RGBA pixel data.
    #[cfg(not(target_os = "macos"))]
    pub fn render_rgba(&self) -> Option<(u32, u32, Vec<u8>)> {
        None // SF Symbols only available on macOS
    }

    /// Render the symbol to a GPUI RenderImage.
    ///
    /// Returns `None` if the symbol cannot be found or rendered.
    ///
    /// Requires the `gpui` feature.
    #[cfg(all(target_os = "macos", feature = "gpui"))]
    pub fn render(&self) -> Option<Arc<RenderImage>> {
        let (width, height, mut data) = self.render_rgba()?;

        // Convert RGBA to BGRA for GPUI
        rgba_to_bgra(&mut data);

        let rgba_image = RgbaImage::from_raw(width, height, data)?;
        let frame = Frame::new(rgba_image);

        Some(Arc::new(RenderImage::new(smallvec![frame])))
    }

    /// Render the symbol to a GPUI RenderImage.
    #[cfg(all(not(target_os = "macos"), feature = "gpui"))]
    pub fn render(&self) -> Option<Arc<RenderImage>> {
        None
    }
}

/// Convert RGBA to BGRA (required by GPUI's Metal renderer).
#[cfg(feature = "gpui")]
fn rgba_to_bgra(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
}

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
/// let icon = Icon::new(MyIcon::Star).text_color(0xFF0000);
/// ```
#[cfg(feature = "component")]
pub trait IconName: Clone + 'static {
    /// Returns the SF Symbol name for this icon.
    fn name(&self) -> &'static str;
}

/// Implement IconName for &'static str for convenience.
#[cfg(feature = "component")]
impl IconName for &'static str {
    fn name(&self) -> &'static str {
        self
    }
}

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
///         .child(Icon::new("heart.fill").text_color(0xFF0000).with_size(px(24.)))
/// }
/// ```
#[cfg(feature = "component")]
#[derive(Clone, IntoElement)]
pub struct Icon {
    name: SharedString,
    size: Pixels,
    color: u32,
}

#[cfg(feature = "component")]
impl Icon {
    /// Create a new Icon with the given SF Symbol name.
    ///
    /// Default size is 16px, default color is black (0x000000).
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            size: px(16.),
            color: 0x000000,
        }
    }

    /// Create a new Icon from a type implementing [`IconName`].
    pub fn from_name<T: IconName>(icon: T) -> Self {
        Self::new(icon.name())
    }

    /// Set the size of the icon in pixels.
    pub fn with_size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// Set the color of the icon as an RGB hex value.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// Icon::new("star.fill").text_color(0xFF0000) // Red
    /// ```
    pub fn text_color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    /// Render the icon to an image.
    fn render_image(&self) -> Option<Arc<RenderImage>> {
        let size: f32 = self.size.into();

        SfSymbol::new(self.name.as_ref())
            .size(size)
            .color(self.color)
            .render()
    }
}

#[cfg(feature = "component")]
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
#[cfg(feature = "component")]
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

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a running GUI environment (AppKit)
    // Run with: cargo test -- --ignored

    #[test]
    #[ignore = "requires GUI environment"]
    #[cfg(target_os = "macos")]
    fn test_render_rgba() {
        let result = SfSymbol::new("star.fill").render_rgba();
        assert!(result.is_some());

        let (width, height, data) = result.unwrap();
        assert!(width > 0);
        assert!(height > 0);
        assert_eq!(data.len(), (width * height * 4) as usize);
    }

    #[test]
    #[ignore = "requires GUI environment"]
    #[cfg(target_os = "macos")]
    fn test_invalid_symbol() {
        let result = SfSymbol::new("this.symbol.does.not.exist.12345").render_rgba();
        assert!(result.is_none());
    }

    #[test]
    #[ignore = "requires GUI environment"]
    #[cfg(target_os = "macos")]
    fn test_custom_color() {
        let result = SfSymbol::new("star.fill")
            .color(0xFF0000)
            .render_rgba();
        assert!(result.is_some());
    }

    #[test]
    #[ignore = "requires GUI environment"]
    #[cfg(all(target_os = "macos", feature = "gpui"))]
    fn test_render_gpui() {
        let result = SfSymbol::new("star.fill").render();
        assert!(result.is_some());
    }

    #[test]
    fn test_builder() {
        let symbol = SfSymbol::new("star.fill")
            .size(48.0)
            .scale(3.0)
            .color(0xFF0000);
        assert_eq!(symbol.size, 48.0);
        assert_eq!(symbol.scale, 3.0);
        assert_eq!(symbol.color, 0xFF0000);
    }
}
