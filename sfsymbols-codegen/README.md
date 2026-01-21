# sfsymbols-codegen

Code generator for [sfsymbols](../sfsymbols). Parses Apple's `name_availability.plist` and generates Rust enums.

## Requirements

- macOS with [SF Symbols.app](https://developer.apple.com/sf-symbols/) installed (free from App Store)

## Usage

```bash
# Default: reads from SF Symbols.app, outputs to ../sfsymbols/src
cargo run

# Custom paths
cargo run -- /path/to/name_availability.plist /path/to/output
```

## Output

Generates:
- **Major version files** (`v1.rs` - `v7.rs`): Merged enums with all symbols from each major release
- **Minor version files** (`v1_0.rs`, `v6_4.rs`, etc.): Precise enums for each point release
- **lib.rs**: Module declarations with feature gates

## Version Mapping

| plist Year | SF Symbols Version | File |
|------------|-------------------|------|
| 2019 | 1.0 | v1_0.rs |
| 2020 | 2.0 | v2_0.rs |
| 2020.1 | 2.1 | v2_1.rs |
| ... | ... | ... |
| 2024 | 6.0 | v6_0.rs |
| 2024.4 | 6.4 | v6_4.rs |
| 2025 | 7.0 | v7_0.rs |

## Naming Conversion

| SF Symbol | Rust Identifier |
|-----------|-----------------|
| `star.fill` | `StarFill` |
| `0.circle` | `N0Circle` |
| `arrow.2.circlepath` | `Arrow2Circlepath` |
| `return` | `r#Return` |

## License

MIT OR Apache-2.0
