# sfsymbols

Type-safe SF Symbols enums for Rust. All 9000+ SF Symbols as compile-time checked enums.

## Installation

```toml
[dependencies]
sfsymbols = "0.1"
```

## Usage

```rust
use sfsymbols::{SfSymbol, SfSymbolV7};

// Use the latest version alias
let name = SfSymbol::StarFill.name();  // "star.fill"

// Or use a specific version
let name = SfSymbolV7::HeartFill.name();  // "heart.fill"

// Works with gpui-symbols
use gpui_symbols::Icon;
let icon = Icon::new(SfSymbol::StarFill.name());
```

## Features

### Major Versions (Merged)

Each major version enum contains all symbols from that release, including minor updates:

| Feature | Enum | Symbols |
|---------|------|---------|
| `v1` | `SfSymbolV1` | 1679 |
| `v2` | `SfSymbolV2` | 1207 |
| `v3` | `SfSymbolV3` | 875 |
| `v4` | `SfSymbolV4` | 1337 |
| `v5` | `SfSymbolV5` | 1486 |
| `v6` | `SfSymbolV6` | 1955 |
| `v7` | `SfSymbolV7` | 645 |

### Minor Versions (Precise)

For precise control, use minor version features:

```toml
[dependencies]
sfsymbols = { version = "0.1", default-features = false, features = ["v6-0", "v6-4"] }
```

Available: `v1-0`, `v1-1`, `v2-0`...`v2-2`, `v3-0`...`v3-3`, `v4-0`...`v4-2`, `v5-0`...`v5-4`, `v6-0`...`v6-4`, `v7-0`, `v7-1`

### All Versions

```toml
sfsymbols = { version = "0.1", features = ["all"] }
```

## API

Each enum implements:

- `name(&self) -> &'static str` - Returns the SF Symbol name
- `AsRef<str>` - Converts to string slice
- `Display` - For formatting
- `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, `Hash`

## Regenerating

To regenerate the enums from the latest SF Symbols.app:

```bash
# Requires SF Symbols.app from App Store
cargo run -p sfsymbols-codegen
```

## License

MIT OR Apache-2.0
