# gpui-symbols

macOS SF Symbols rendering for Rust / GPUI applications.

## Installation

```toml
[dependencies]
gpui-symbols = "0.1"

# With GPUI integration
gpui-symbols = { version = "0.1", features = ["gpui"] }
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

## API

### `SfSymbol`

Builder for rendering SF Symbols.

| Method | Description | Default |
|--------|-------------|---------|
| `new(name)` | Create symbol with name | - |
| `size(f32)` | Point size | 32.0 |
| `scale(f32)` | Scale factor (Retina) | 2.0 |
| `color(u32)` | RGB hex color | 0x000000 |
| `render_rgba()` | Render to `(width, height, Vec<u8>)` | - |
| `render()` | Render to `Arc<RenderImage>` (requires `gpui` feature) | - |

## Features

| Feature | Description |
|---------|-------------|
| `default` | Core rendering, returns raw RGBA pixels |
| `gpui` | GPUI integration, returns `Arc<RenderImage>` |

## Requirements

- macOS 11.0+ (SF Symbols support)
- Rust 1.75+

## Examples

Run the GPUI example:

```bash
cargo run --example basic --features gpui
```

## Symbol Names

See [SF Symbols](https://developer.apple.com/sf-symbols/) for available symbol names.

Common symbols:
- `star.fill`, `heart.fill`, `house.fill`
- `gearshape`, `magnifyingglass`, `trash`
- `person.fill`, `folder.fill`, `doc.fill`

## License

MIT OR Apache-2.0
