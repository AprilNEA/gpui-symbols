# gpui-symbols

macOS SF Symbols rendering for Rust / GPUI applications.

## Installation

```toml
[dependencies]
gpui-symbols = "0.2"

# With GPUI integration
gpui-symbols = { version = "0.2", features = ["gpui"] }

# With Icon component (recommended for GPUI apps)
gpui-symbols = { version = "0.2", features = ["component"] }
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

## API

### `SfSymbol`

Low-level builder for rendering SF Symbols.

| Method | Description | Default |
|--------|-------------|---------|
| `new(name)` | Create symbol with name | - |
| `size(f32)` | Point size | 32.0 |
| `scale(f32)` | Scale factor (Retina) | 2.0 |
| `color(u32)` | RGB hex color | 0x000000 |
| `render_rgba()` | Render to `(width, height, Vec<u8>)` | - |
| `render()` | Render to `Arc<RenderImage>` (requires `gpui` feature) | - |

### `Icon`

High-level GPUI component (requires `component` feature).

| Method | Description | Default |
|--------|-------------|---------|
| `new(name)` | Create icon with SF Symbol name | - |
| `from_name(T)` | Create from `IconName` type | - |
| `size(Pixels)` | Set icon size | 16px |
| `text_color(u32)` | Set RGB hex color | 0x000000 |

## Features

| Feature | Description |
|---------|-------------|
| `default` | `component` + `presets` |
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
