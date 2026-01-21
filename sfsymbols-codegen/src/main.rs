//! SF Symbols code generator
//!
//! Generates both major version enums (merged) and minor version enums.

use heck::ToUpperCamelCase;
use plist::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Parse year string to (major, minor) version tuple
fn year_to_version(year: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = year.split('.').collect();
    let base_year = parts.first()?;
    let minor = parts.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);

    let major = match *base_year {
        "2019" => 1,
        "2020" => 2,
        "2021" => 3,
        "2022" => 4,
        "2023" => 5,
        "2024" => 6,
        "2025" => 7,
        _ => return None,
    };

    Some((major, minor))
}

fn to_rust_identifier(name: &str) -> String {
    let parts: Vec<&str> = name.split('.').collect();
    let mut result = String::new();

    for (i, part) in parts.iter().enumerate() {
        let first_char = part.chars().next();
        let starts_with_digit = first_char.map(|c| c.is_ascii_digit()).unwrap_or(false);

        if i == 0 && starts_with_digit {
            result.push('N');
            result.push_str(&capitalize_part(part));
        } else if starts_with_digit {
            result.push_str(&capitalize_part(part));
        } else {
            result.push_str(&part.to_upper_camel_case());
        }
    }

    let lower = result.to_lowercase();
    if RUST_KEYWORDS.contains(&lower.as_str()) {
        format!("r#{}", result)
    } else {
        result
    }
}

fn capitalize_part(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_ascii_digit() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    rust_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plist_path = std::env::args().nth(1).unwrap_or_else(|| {
        "../SFSafeSymbols/SymbolsGenerator/Sources/SymbolsGenerator/Resources/name_availability.plist".to_string()
    });
    let output_dir = std::env::args().nth(2).unwrap_or_else(|| "../src".to_string());

    println!("Reading plist from: {}", plist_path);

    let plist_content = fs::read(&plist_path)?;
    let plist: Value = plist::from_bytes(&plist_content)?;

    let symbols_dict = plist
        .as_dictionary()
        .and_then(|d| d.get("symbols"))
        .and_then(|v| v.as_dictionary())
        .ok_or("Invalid plist structure")?;

    // Group symbols by (major, minor) version
    let mut by_minor: BTreeMap<(u32, u32), Vec<Symbol>> = BTreeMap::new();

    for (name, value) in symbols_dict {
        if let Some(year_str) = value.as_string() {
            if let Some(version) = year_to_version(year_str) {
                let rust_name = to_rust_identifier(name);
                let symbol = Symbol {
                    name: name.clone(),
                    rust_name,
                };
                by_minor.entry(version).or_default().push(symbol);
            }
        }
    }

    // Sort symbols
    for symbols in by_minor.values_mut() {
        symbols.sort_by(|a, b| a.rust_name.cmp(&b.rust_name));
    }

    // Group by major version (merge all minors)
    let mut by_major: BTreeMap<u32, Vec<Symbol>> = BTreeMap::new();
    for ((major, _), symbols) in &by_minor {
        by_major.entry(*major).or_default().extend(symbols.clone());
    }
    for symbols in by_major.values_mut() {
        symbols.sort_by(|a, b| a.rust_name.cmp(&b.rust_name));
    }

    let output_path = Path::new(&output_dir);
    fs::create_dir_all(output_path)?;

    // Collect all versions
    let minor_versions: Vec<_> = by_minor.keys().cloned().collect();
    let major_versions: Vec<_> = by_major.keys().cloned().collect();

    // Generate lib.rs
    let mut lib = String::from(
        "//! SF Symbols presets - type-safe enums for all SF Symbols versions.\n\
         //!\n\
         //! # Features\n\
         //!\n\
         //! - **Major versions** (`v1` - `v7`): Merged enums containing all symbols from that major release\n\
         //! - **Minor versions** (`v1-0`, `v6-4`, etc.): Precise enums for specific point releases\n\
         //!\n\
         //! ```toml\n\
         //! # Latest major version (default)\n\
         //! sfsymbols = \"0.1\"\n\
         //!\n\
         //! # Specific major version\n\
         //! sfsymbols = { version = \"0.1\", features = [\"v6\"] }\n\
         //!\n\
         //! # Specific minor versions\n\
         //! sfsymbols = { version = \"0.1\", default-features = false, features = [\"v6-0\", \"v6-4\"] }\n\
         //!\n\
         //! # All versions\n\
         //! sfsymbols = { version = \"0.1\", features = [\"all\"] }\n\
         //! ```\n\n",
    );

    // Minor version modules
    for (major, minor) in &minor_versions {
        lib.push_str(&format!("#[cfg(feature = \"v{}-{}\")]\n", major, minor));
        lib.push_str(&format!("mod v{}_{};\n", major, minor));
    }
    lib.push('\n');

    // Major version modules
    for major in &major_versions {
        lib.push_str(&format!("#[cfg(feature = \"v{}\")]\n", major));
        lib.push_str(&format!("mod v{};\n", major));
    }
    lib.push('\n');

    // Re-exports for minor versions
    for (major, minor) in &minor_versions {
        lib.push_str(&format!("#[cfg(feature = \"v{}-{}\")]\n", major, minor));
        lib.push_str(&format!(
            "pub use v{}_{}::SfSymbolV{}_{};\n",
            major, minor, major, minor
        ));
    }
    lib.push('\n');

    // Re-exports for major versions
    for major in &major_versions {
        lib.push_str(&format!("#[cfg(feature = \"v{}\")]\n", major));
        lib.push_str(&format!("pub use v{}::SfSymbolV{};\n", major, major));
    }

    // Alias for latest
    if let Some(&latest) = major_versions.last() {
        lib.push_str(&format!(
            "\n/// Alias for the latest SF Symbols version (SfSymbolV{})\n",
            latest
        ));
        lib.push_str(&format!("#[cfg(feature = \"v{}\")]\n", latest));
        lib.push_str(&format!("pub type SfSymbol = SfSymbolV{};\n", latest));
    }

    fs::write(output_path.join("lib.rs"), &lib)?;
    println!("Generated lib.rs");

    // Generate minor version files
    for ((major, minor), symbols) in &by_minor {
        let enum_name = format!("SfSymbolV{}_{}", major, minor);
        let content = generate_enum_file(&enum_name, symbols);
        let filename = format!("v{}_{}.rs", major, minor);
        fs::write(output_path.join(&filename), &content)?;
        println!("Generated {} ({} symbols)", filename, symbols.len());
    }

    // Generate major version files
    for (major, symbols) in &by_major {
        let enum_name = format!("SfSymbolV{}", major);
        let content = generate_enum_file(&enum_name, symbols);
        let filename = format!("v{}.rs", major);
        fs::write(output_path.join(&filename), &content)?;
        println!("Generated {} ({} symbols, merged)", filename, symbols.len());
    }

    // Print features for Cargo.toml
    println!("\n# Cargo.toml [features]:");
    println!("default = [\"v{}\"]", major_versions.last().unwrap_or(&7));

    // Major version features
    for major in &major_versions {
        println!("v{} = []", major);
    }

    // Minor version features
    for (major, minor) in &minor_versions {
        println!("v{}-{} = []", major, minor);
    }

    // All features
    let mut all_features = Vec::new();
    for major in &major_versions {
        all_features.push(format!("\"v{}\"", major));
    }
    for (major, minor) in &minor_versions {
        all_features.push(format!("\"v{}-{}\"", major, minor));
    }
    println!("all = [{}]", all_features.join(", "));

    let total_minor: usize = by_minor.values().map(|v| v.len()).sum();
    println!(
        "\nTotal: {} symbols, {} major versions, {} minor versions",
        total_minor,
        major_versions.len(),
        minor_versions.len()
    );

    Ok(())
}

fn generate_enum_file(enum_name: &str, symbols: &[Symbol]) -> String {
    let mut content = String::new();

    content.push_str("//! Auto-generated SF Symbols enum.\n");
    content.push_str("//! Do not edit manually - regenerate with codegen.\n\n");

    content.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n");
    content.push_str("#[non_exhaustive]\n");
    content.push_str(&format!("pub enum {} {{\n", enum_name));

    for symbol in symbols {
        content.push_str(&format!("    /// `{}`\n", symbol.name));
        content.push_str(&format!("    {},\n", symbol.rust_name));
    }

    content.push_str("}\n\n");

    content.push_str(&format!("impl {} {{\n", enum_name));
    content.push_str("    /// Returns the SF Symbol name string.\n");
    content.push_str("    #[inline]\n");
    content.push_str("    pub const fn name(&self) -> &'static str {\n");
    content.push_str("        match self {\n");

    for symbol in symbols {
        content.push_str(&format!(
            "            Self::{} => \"{}\",\n",
            symbol.rust_name, symbol.name
        ));
    }

    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");

    content.push_str(&format!("impl AsRef<str> for {} {{\n", enum_name));
    content.push_str("    #[inline]\n");
    content.push_str("    fn as_ref(&self) -> &str {\n");
    content.push_str("        self.name()\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");

    content.push_str(&format!("impl std::fmt::Display for {} {{\n", enum_name));
    content.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    content.push_str("        f.write_str(self.name())\n");
    content.push_str("    }\n");
    content.push_str("}\n");

    content
}
