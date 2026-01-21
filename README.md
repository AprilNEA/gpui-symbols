# gpui-symbols

[![Crates.io](https://img.shields.io/crates/v/gpui-symbols.svg)](https://crates.io/crates/gpui-symbols)
[![Documentation](https://docs.rs/gpui-symbols/badge.svg)](https://docs.rs/gpui-symbols)
[![License](https://img.shields.io/crates/l/gpui-symbols.svg)](https://github.com/AprilNEA/gpui-symbols#license)
[![Downloads](https://img.shields.io/crates/d/gpui-symbols.svg)](https://crates.io/crates/gpui-symbols)

macOS SF Symbols rendering for Rust / GPUI applications.

## Installation

```toml
[dependencies]
gpui-symbols = "0.6"

# With GPUI integration
gpui-symbols = { version = "0.7", features = ["gpui"] }

# With Icon component (recommended for GPUI apps)
gpui-symbols = { version = "0.7", features = ["component"] }
```

## Usage

### Basic - Raw RGBA Pixels

```rust
use gpui_symbols::SfSymbol;

let (width, height, rgba_data) = SfSymbol::new("star.fill")
    .size(32.0)
    .color(0x000000)
    .render_rgba()
    .unwrap();
```

### With GPUI Integration

```rust
use gpui_symbols::SfSymbol;
use gpui::{img, ImageSource};

let image = SfSymbol::new("heart.fill")
    .size(48.0)
    .color(0xFF0000)
    .render()
    .unwrap();

// Use in GPUI element
img(ImageSource::Render(image)).size(px(32.))
```

### Icon Component (Recommended)

The `Icon` component provides a high-level API similar to GPUI Components:

```rust
use gpui_symbols::Icon;
use gpui::px;

fn view(window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    div()
        .child(Icon::new("star.fill"))
        .child(Icon::new("heart.fill").text_color(0xFF0000).size(px(24.)))
}
```

#### Using Preset Symbols (Recommended)

With the `presets` feature, use type-safe SF Symbols enums directly:

```rust
use gpui_symbols::{Icon, sfsymbols::SfSymbolV7};

// All 9000+ SF Symbols as compile-time checked enums
let icon = Icon::from_name(SfSymbolV7::StarFill);
let heart = Icon::from_name(SfSymbolV7::HeartFill).text_color(0xFF0000);
```

#### Using Unified Symbol Enum (New in 0.6)

Use `SfSymbol` for a single type covering all SF Symbols versions:

```rust
use gpui_symbols::{Icon, sfsymbols::SfSymbol};

// Unified enum with version metadata
let icon = Icon::from_name(SfSymbol::Gearshape);

// Check minimum version required
let (major, minor) = SfSymbol::Gearshape.min_version(); // (2, 0)
```

This is useful for cross-platform abstractions:

```rust
use gpui_symbols::sfsymbols::SfSymbol;

enum AppIcon { Settings, Search, Plus }

impl AppIcon {
    fn sf_symbol(&self) -> SfSymbol {
        match self {
            Self::Settings => SfSymbol::Gearshape,
            Self::Search => SfSymbol::Magnifyingglass,
            Self::Plus => SfSymbol::Plus,
        }
    }
}
```

#### Define Custom Icon Enums

Alternatively, use the `define_icons!` macro:

```rust
use gpui_symbols::{Icon, define_icons};

define_icons! {
    pub enum AppIcon {
        Star => "star.fill",
        Heart => "heart.fill",
        Settings => "gearshape.fill",
    }
}

let icon = Icon::from_name(AppIcon::Star).text_color(0xFF0000);
```

### Aspect Ratio Preservation (New in 0.6.1)

SF Symbols are not always square - for example, `gearshape.2` is wider than tall. The `Icon` component automatically preserves the original aspect ratio:

```rust
// Non-square symbols render correctly
Icon::new("gearshape").size(px(32.))     // Square gear
Icon::new("gearshape.2").size(px(32.))   // Wide: two gears side by side
Icon::new("person.3.fill").size(px(32.)) // Wide: three people
```

The `size()` method sets the maximum dimension while maintaining the symbol's natural proportions.

### Advanced Symbol Options

Customize weight, scale, and rendering mode:

```rust
use gpui_symbols::{Icon, SymbolWeight, SymbolScale, RenderingMode};

// Bold weight icon
Icon::new("star.fill")
    .weight(SymbolWeight::Bold)
    .size(px(24.));

// Large scale with multicolor rendering
Icon::new("cloud.sun.fill")
    .symbol_scale(SymbolScale::Large)
    .rendering_mode(RenderingMode::Multicolor);

// Thin weight, monochrome style
Icon::new("heart.fill")
    .weight(SymbolWeight::Thin)
    .rendering_mode(RenderingMode::Monochrome)
    .text_color(0xFF0000);
```

### Integration with gpui-component

Use SF Symbols with [gpui-component](https://github.com/longbridge/gpui-component) Button via `child()`:

```rust
use gpui_symbols::{Icon as SfIcon, sfsymbols::SfSymbolV7};
use gpui_component::Button;

// Icon-only button
Button::new("star-btn")
    .child(SfIcon::from_name(SfSymbolV7::StarFill).size(px(16.)))

// Icon + label button
Button::new("favorite-btn")
    .child(
        div().flex().items_center().gap_2()
            .child(SfIcon::from_name(SfSymbolV7::HeartFill).size(px(16.)).text_color(0xFF0000))
            .child("Favorite")
    )
```

> **Note**: gpui-component's `Button::icon()` expects SVG paths, while gpui-symbols renders to pixel images. Use `Button::child()` for SF Symbols integration.

## API

### `SfSymbol`

Low-level builder for rendering SF Symbols.

| Method | Description | Default |
|--------|-------------|---------|
| `new(name)` | Create symbol with name | - |
| `size(f32)` | Point size | 32.0 |
| `scale(f32)` | Scale factor (Retina) | 2.0 |
| `color(u32)` | RGB hex color | 0x000000 |
| `weight(SymbolWeight)` | Symbol weight | Regular |
| `symbol_scale(SymbolScale)` | Symbol scale | Medium |
| `rendering_mode(RenderingMode)` | Rendering mode | Hierarchical |
| `render_rgba()` | Render to `(width, height, Vec<u8>)` | - |
| `render()` | Render to `Arc<RenderImage>` (requires `gpui` feature) | - |

### `Icon`

High-level GPUI component (requires `component` feature).

| Method | Description | Default |
|--------|-------------|---------|
| `new(name)` | Create icon with SF Symbol name | - |
| `from_name(T)` | Create from `IconName` type | - |
| `size(Pixels)` | Set maximum dimension (preserves aspect ratio) | 16px |
| `color(impl Into<Hsla>)` | Set color (Hsla, Rgba, or `rgb(hex)`) | black |
| `text_color(u32)` | Set RGB hex color (convenience) | black |
| `weight(SymbolWeight)` | Symbol weight | Regular |
| `symbol_scale(SymbolScale)` | Symbol scale | Medium |
| `rendering_mode(RenderingMode)` | Rendering mode | Hierarchical |

### Enums

**`SymbolWeight`**: `UltraLight`, `Thin`, `Light`, `Regular`, `Medium`, `Semibold`, `Bold`, `Heavy`, `Black`

**`SymbolScale`**: `Small`, `Medium`, `Large`

**`RenderingMode`**: `Monochrome`, `Hierarchical`, `Palette`, `Multicolor`

### Cache Management

With the `cache` feature (enabled by default), rendered symbols are cached globally for performance:

```rust
use gpui_symbols::{cache_size, clear_cache};

// Get number of cached symbols
let count = cache_size();

// Clear all cached symbols (e.g., on memory pressure or appearance change)
clear_cache();
```

## Features

| Feature | Description |
|---------|-------------|
| `default` | `component` + `presets` + `cache` |
| `cache` | Global cache for rendered symbols (implies `gpui`) |
| `gpui` | GPUI integration, returns `Arc<RenderImage>` |
| `component` | High-level `Icon` component (implies `gpui`) |
| `presets` | Type-safe SF Symbols enums via `sfsymbols` crate |

## Requirements

- macOS 11.0+ (SF Symbols support)
- Rust 1.75+

## Examples

Run the example:

```bash
cargo run --example basic --features component
```

## Symbol Names

See [SF Symbols](https://developer.apple.com/sf-symbols/) for available symbol names.

Common symbols:
- `star.fill`, `heart.fill`, `house.fill`
- `gearshape`, `magnifyingglass`, `trash`
- `person.fill`, `folder.fill`, `doc.fill`

## Related Crates

| Crate | Description |
|-------|-------------|
| [sfsymbols](./sfsymbols) | Type-safe SF Symbols enums (9000+ symbols) |
| [sfsymbols-codegen](./sfsymbols-codegen) | Code generator for sfsymbols |

## License

MIT OR Apache-2.0
