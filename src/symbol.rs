//! SF Symbol rendering to raw RGBA or GPUI RenderImage.

use crate::platform;

#[cfg(feature = "cache")]
use std::collections::HashMap;
#[cfg(feature = "cache")]
use std::sync::{Mutex, OnceLock};

/// Cache key for rendered symbols.
#[cfg(feature = "cache")]
#[derive(Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    name: String,
    size_bits: u32,
    scale_bits: u32,
    color: u32,
    weight: u8,
    symbol_scale: u8,
    rendering_mode: u8,
}

#[cfg(feature = "cache")]
impl CacheKey {
    fn new(
        name: &str,
        size: f32,
        scale: f32,
        color: u32,
        weight: SymbolWeight,
        symbol_scale: SymbolScale,
        rendering_mode: RenderingMode,
    ) -> Self {
        Self {
            name: name.to_string(),
            size_bits: size.to_bits(),
            scale_bits: scale.to_bits(),
            color,
            weight: weight as u8,
            symbol_scale: symbol_scale as u8,
            rendering_mode: rendering_mode as u8,
        }
    }
}

/// Global cache for rendered symbols.
/// Stores (image, width, height) to preserve aspect ratio information.
#[cfg(feature = "cache")]
type SymbolCache = HashMap<CacheKey, (std::sync::Arc<gpui::RenderImage>, u32, u32)>;

#[cfg(feature = "cache")]
static SYMBOL_CACHE: OnceLock<Mutex<SymbolCache>> = OnceLock::new();

#[cfg(feature = "cache")]
fn get_cache() -> &'static Mutex<SymbolCache> {
    SYMBOL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Clear the global symbol cache.
///
/// Call this if you need to free memory or if system appearance changed.
#[cfg(feature = "cache")]
pub fn clear_cache() {
    if let Some(cache) = SYMBOL_CACHE.get() {
        if let Ok(mut guard) = cache.lock() {
            guard.clear();
        }
    }
}

/// Get the number of cached symbols.
#[cfg(feature = "cache")]
pub fn cache_size() -> usize {
    SYMBOL_CACHE
        .get()
        .and_then(|c| c.lock().ok())
        .map(|g| g.len())
        .unwrap_or(0)
}

/// Symbol weight for SF Symbols.
///
/// Corresponds to NSFontWeight values used in NSImageSymbolConfiguration.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SymbolWeight {
    /// Ultra light weight (-0.8)
    UltraLight,
    /// Thin weight (-0.6)
    Thin,
    /// Light weight (-0.4)
    Light,
    /// Regular weight (0.0) - default
    #[default]
    Regular,
    /// Medium weight (0.23)
    Medium,
    /// Semibold weight (0.3)
    Semibold,
    /// Bold weight (0.4)
    Bold,
    /// Heavy weight (0.56)
    Heavy,
    /// Black weight (0.62)
    Black,
}

impl SymbolWeight {
    /// Convert to NSFontWeight value.
    pub fn to_ns_weight(self) -> f64 {
        match self {
            Self::UltraLight => -0.8,
            Self::Thin => -0.6,
            Self::Light => -0.4,
            Self::Regular => 0.0,
            Self::Medium => 0.23,
            Self::Semibold => 0.3,
            Self::Bold => 0.4,
            Self::Heavy => 0.56,
            Self::Black => 0.62,
        }
    }
}

/// Symbol scale for SF Symbols.
///
/// Corresponds to NSImageSymbolScale values.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SymbolScale {
    /// Small scale (1)
    Small,
    /// Medium scale (2) - default
    #[default]
    Medium,
    /// Large scale (3)
    Large,
}

impl SymbolScale {
    /// Convert to NSImageSymbolScale value.
    pub fn to_ns_scale(self) -> i64 {
        match self {
            Self::Small => 1,
            Self::Medium => 2,
            Self::Large => 3,
        }
    }
}

/// Rendering mode for SF Symbols.
///
/// Determines how the symbol is colorized.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum RenderingMode {
    /// Single color applied to entire symbol
    Monochrome,
    /// Hierarchical coloring with primary color and automatic opacity layers
    #[default]
    Hierarchical,
    /// Multiple colors (requires palette colors, falls back to hierarchical)
    Palette,
    /// Original multicolor design of the symbol
    Multicolor,
}

#[cfg(feature = "gpui")]
use std::sync::Arc;

#[cfg(feature = "gpui")]
use gpui::RenderImage;
#[cfg(feature = "gpui")]
use image::{Frame, RgbaImage};
#[cfg(feature = "gpui")]
use smallvec::smallvec;

/// SF Symbol builder for rendering system symbols.
///
/// # Example
///
/// ```rust,ignore
/// use gpui_symbols::{SfSymbol, SymbolWeight, SymbolScale, RenderingMode};
///
/// // Render to raw RGBA
/// let (width, height, data) = SfSymbol::new("star.fill")
///     .render_rgba()
///     .unwrap();
///
/// // With gpui feature and advanced options:
/// let image = SfSymbol::new("heart.fill")
///     .size(48.0)
///     .color(0xFF0000)
///     .weight(SymbolWeight::Bold)
///     .symbol_scale(SymbolScale::Large)
///     .rendering_mode(RenderingMode::Hierarchical)
///     .render()
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct SfSymbol {
    name: String,
    size: f32,
    scale: f32,
    color: u32,
    weight: SymbolWeight,
    symbol_scale: SymbolScale,
    rendering_mode: RenderingMode,
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
            weight: SymbolWeight::default(),
            symbol_scale: SymbolScale::default(),
            rendering_mode: RenderingMode::default(),
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

    /// Set the symbol weight (default: Regular).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui_symbols::{SfSymbol, SymbolWeight};
    ///
    /// SfSymbol::new("star.fill")
    ///     .weight(SymbolWeight::Bold)
    ///     .render();
    /// ```
    pub fn weight(mut self, weight: SymbolWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set the symbol scale (default: Medium).
    ///
    /// This controls the overall visual weight/size of the symbol
    /// independent of the point size.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui_symbols::{SfSymbol, SymbolScale};
    ///
    /// SfSymbol::new("star.fill")
    ///     .symbol_scale(SymbolScale::Large)
    ///     .render();
    /// ```
    pub fn symbol_scale(mut self, symbol_scale: SymbolScale) -> Self {
        self.symbol_scale = symbol_scale;
        self
    }

    /// Set the rendering mode (default: Hierarchical).
    ///
    /// - `Monochrome`: Single color for the entire symbol
    /// - `Hierarchical`: Primary color with automatic opacity layers
    /// - `Palette`: Multiple distinct colors (requires palette support)
    /// - `Multicolor`: Original multicolor design
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use gpui_symbols::{SfSymbol, RenderingMode};
    ///
    /// SfSymbol::new("cloud.sun.fill")
    ///     .rendering_mode(RenderingMode::Multicolor)
    ///     .render();
    /// ```
    pub fn rendering_mode(mut self, mode: RenderingMode) -> Self {
        self.rendering_mode = mode;
        self
    }

    /// Render to raw RGBA pixel data.
    ///
    /// Returns `(width, height, rgba_data)` or `None` if rendering fails.
    pub fn render_rgba(&self) -> Option<(u32, u32, Vec<u8>)> {
        platform::render_sf_symbol(
            &self.name,
            self.size,
            self.scale,
            self.color,
            self.weight,
            self.symbol_scale,
            self.rendering_mode,
        )
    }

    /// Render the symbol to a GPUI RenderImage.
    ///
    /// With the `cache` feature enabled, results are cached globally
    /// to avoid redundant rendering. Use [`clear_cache`] to free memory if needed.
    ///
    /// Returns `None` if the symbol cannot be found or rendered.
    ///
    /// Requires the `gpui` feature.
    #[cfg(all(feature = "gpui", feature = "cache"))]
    pub fn render(&self) -> Option<Arc<RenderImage>> {
        self.render_with_size().map(|(image, _, _)| image)
    }

    /// Render the symbol to a GPUI RenderImage with its actual dimensions.
    ///
    /// Returns `(image, width, height)` where width and height are the actual
    /// rendered pixel dimensions. SF Symbols may not be square - use these
    /// dimensions to preserve the correct aspect ratio when displaying.
    ///
    /// With the `cache` feature enabled, results are cached globally
    /// to avoid redundant rendering. Use [`clear_cache`] to free memory if needed.
    ///
    /// Returns `None` if the symbol cannot be found or rendered.
    ///
    /// Requires the `gpui` feature.
    #[cfg(all(feature = "gpui", feature = "cache"))]
    pub fn render_with_size(&self) -> Option<(Arc<RenderImage>, u32, u32)> {
        let cache_key = CacheKey::new(
            &self.name,
            self.size,
            self.scale,
            self.color,
            self.weight,
            self.symbol_scale,
            self.rendering_mode,
        );

        // Check cache first
        {
            let cache = get_cache();
            if let Ok(guard) = cache.lock() {
                if let Some((cached, w, h)) = guard.get(&cache_key) {
                    return Some((Arc::clone(cached), *w, *h));
                }
            }
        }

        // Render and cache
        let (width, height, mut data) = self.render_rgba()?;

        // Convert RGBA to BGRA for GPUI's Metal renderer
        rgba_to_bgra(&mut data);

        let rgba_image = RgbaImage::from_raw(width, height, data)?;
        let frame = Frame::new(rgba_image);
        let image = Arc::new(RenderImage::new(smallvec![frame]));

        // Store in cache
        {
            let cache = get_cache();
            if let Ok(mut guard) = cache.lock() {
                guard.insert(cache_key, (Arc::clone(&image), width, height));
            }
        }

        Some((image, width, height))
    }

    /// Render the symbol to a GPUI RenderImage.
    ///
    /// Returns `None` if the symbol cannot be found or rendered.
    ///
    /// Requires the `gpui` feature.
    #[cfg(all(feature = "gpui", not(feature = "cache")))]
    pub fn render(&self) -> Option<Arc<RenderImage>> {
        self.render_with_size().map(|(image, _, _)| image)
    }

    /// Render the symbol to a GPUI RenderImage with its actual dimensions.
    ///
    /// Returns `(image, width, height)` where width and height are the actual
    /// rendered pixel dimensions. SF Symbols may not be square - use these
    /// dimensions to preserve the correct aspect ratio when displaying.
    ///
    /// Returns `None` if the symbol cannot be found or rendered.
    ///
    /// Requires the `gpui` feature.
    #[cfg(all(feature = "gpui", not(feature = "cache")))]
    pub fn render_with_size(&self) -> Option<(Arc<RenderImage>, u32, u32)> {
        let (width, height, mut data) = self.render_rgba()?;

        // Convert RGBA to BGRA for GPUI's Metal renderer
        rgba_to_bgra(&mut data);

        let rgba_image = RgbaImage::from_raw(width, height, data)?;
        let frame = Frame::new(rgba_image);

        Some((Arc::new(RenderImage::new(smallvec![frame])), width, height))
    }
}

/// Convert RGBA to BGRA (required by GPUI's Metal renderer).
#[cfg(feature = "gpui")]
fn rgba_to_bgra(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = SfSymbol::new("star.fill").color(0xFF0000).render_rgba();
        assert!(result.is_some());
    }

    #[test]
    #[ignore = "requires GUI environment"]
    #[cfg(all(target_os = "macos", feature = "gpui"))]
    fn test_render_gpui() {
        let result = SfSymbol::new("star.fill").render();
        assert!(result.is_some());
    }
}
