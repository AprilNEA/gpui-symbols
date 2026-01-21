//! macOS platform implementation using AppKit.

use crate::symbol::{RenderingMode, SymbolScale, SymbolWeight};
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

/// Render SF Symbol to RGBA pixel buffer.
///
/// # Arguments
///
/// * `name` - The SF Symbol name (e.g., "star.fill")
/// * `point_size` - The point size for rendering
/// * `scale` - The scale factor (e.g., 2.0 for Retina)
/// * `color` - RGB hex color value
/// * `weight` - Symbol weight (ultraLight to black)
/// * `symbol_scale` - Symbol scale (small, medium, large)
/// * `rendering_mode` - Rendering mode (monochrome, hierarchical, palette, multicolor)
///
/// # Returns
///
/// Returns `Some((width, height, rgba_data))` on success, `None` on failure.
pub fn render_sf_symbol(
    name: &str,
    point_size: f32,
    scale: f32,
    color: u32,
    weight: SymbolWeight,
    symbol_scale: SymbolScale,
    rendering_mode: RenderingMode,
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

        // 4. Create symbol configuration with size, weight, scale, and color
        let config_class = Class::get("NSImageSymbolConfiguration")?;

        // Size and weight configuration
        let size_config: *mut Object = msg_send![
            config_class,
            configurationWithPointSize: point_size as f64
            weight: weight.to_ns_weight()
            scale: symbol_scale.to_ns_scale()
        ];

        // Color configuration based on rendering mode
        let color_config: *mut Object = match rendering_mode {
            RenderingMode::Monochrome => {
                // For monochrome, we use preferringMonochrome + tint color
                let mono_config: *mut Object =
                    msg_send![config_class, configurationPreferringMonochrome];
                let tint_config: *mut Object =
                    msg_send![config_class, configurationWithHierarchicalColor: symbol_color];
                msg_send![mono_config, configurationByApplyingConfiguration: tint_config]
            }
            RenderingMode::Hierarchical => {
                msg_send![config_class, configurationWithHierarchicalColor: symbol_color]
            }
            RenderingMode::Palette => {
                // Palette mode with single color falls back to hierarchical-like behavior
                // For true palette, would need multiple colors
                msg_send![config_class, configurationWithHierarchicalColor: symbol_color]
            }
            RenderingMode::Multicolor => {
                // Multicolor uses the symbol's built-in colors
                msg_send![config_class, configurationPreferringMulticolor]
            }
        };

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

        // 9. Draw symbol (transparent background - bitmap is zero-initialized)
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

        // 10. Restore context
        let _: () = msg_send![ns_graphics_context_class, setCurrentContext: old_context];

        // 11. Get pixel data
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
