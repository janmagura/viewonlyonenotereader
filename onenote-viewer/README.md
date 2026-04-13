
# OneNote Viewer

A read-only desktop viewer for Microsoft OneNote notebook files (`.one`, `.onetoc2`) targeting Debian-based Linux distributions.

## Features

- 📖 **Read-Only Viewing**: Open and view OneNote notebooks exported from OneDrive/OneNote Web
- 🌳 **Navigation Sidebar**: Browse Notebook → Sections → Pages hierarchy
- 📝 **Rich Content Rendering**:
  - Text with formatting (bold, italic, headers, lists)
  - Embedded images displayed inline
  - Attachments with icons and file information
- 🎨 **Modern UI**: Built with GTK4 and libadwaita for native Linux integration
- 🌓 **Theme Support**: Automatic light/dark mode following system preferences
- ⚠️ **Graceful Degradation**: Skips unsupported content (ink, equations, encrypted pages) with user-friendly warnings

## Installation

### From Source

```bash
# Install dependencies
sudo apt install libgtk-4-dev libadwaita-1-dev cargo rustc

# Clone the repository
git clone https://github.com/yourusername/onenote-viewer.git
cd onenote-viewer

# Build
cargo build --release

# Run
./target/release/onenote-viewer
```

### From .deb Package

Download the latest `.deb` package from the [Releases](https://github.com/yourusername/onenote-viewer/releases) page and install:

```bash
sudo apt install ./onenote-viewer_*.deb
```

## Usage

### GUI Mode
Launch the application and use the file chooser to select a `.one` or `.onetoc2` file:

```bash
onenote-viewer
```

### CLI Mode
Open a file directly from the command line:

```bash
onenote-viewer /path/to/notebook.onetoc2
```

Or open a standalone section file:

```bash
onenote-viewer /path/to/section.one
```

## Supported Formats

| Format | Description | Support Level |
|--------|-------------|---------------|
| `.onetoc2` | OneNote notebook table of contents | ✅ Full |
| `.one` | OneNote section/page files | ✅ Full |
| Legacy formats (2010 and earlier) | Older OneNote formats | ⚠️ Partial |
| Encrypted notebooks | Password-protected notebooks | ❌ Not supported |

## Content Support

| Content Type | Support |
|--------------|---------|
| Text runs | ✅ Yes |
| Bold/Italic formatting | ✅ Yes |
| Headers | ✅ Yes |
| Lists (bulleted/numbered) | ✅ Yes |
| Images | ✅ Yes |
| Attachments | ✅ Yes (open with default app) |
| Ink drawings | ❌ No |
| Equations | ❌ No |
| Audio/Video | ❌ No |
| Tables | ⚠️ Basic support |

## Building .deb Package

```bash
# Install cargo-deb
cargo install cargo-deb

# Build the package
cargo deb

# The .deb file will be in target/debian/
```

## Development

### Project Structure

```
onenote-viewer/
├── Cargo.toml          # Rust package manifest
├── src/
│   ├── main.rs         # Application entry point
│   ├── app.rs          # Main application logic
│   ├── parser.rs       # OneNote file parsing
│   ├── utils.rs        # Utility functions
│   └── ui/
│       ├── mod.rs      # UI module exports
│       ├── sidebar.rs  # Navigation sidebar
│       └── content_view.rs  # Content rendering
├── assets/
│   ├── icons/          # Application icons
│   └── onenote-viewer.desktop  # Desktop entry
├── data/
│   └── com.example.onenote-viewer.metainfo.xml  # AppStream metadata
└── .github/
    └── workflows/
        └── ci.yml      # GitHub Actions CI
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt --check
```

### Linting

```bash
cargo clippy -- -D warnings
```

## Dependencies

- **Rust** (2021 edition)
- **GTK4** (`gtk4 = "0.9"`)
- **libadwaita** (`libadwaita = "0.7"`)
- **onenote_parser** (latest stable)
- **tokio** (async runtime)

## System Requirements

- **OS**: Debian 12+, Ubuntu 22.04+, or compatible
- **Architecture**: amd64, arm64
- **RAM**: 256 MB minimum
- **Disk**: 50 MB for installation

## Limitations

This is a **read-only viewer**. The following features are intentionally not supported:

- Editing or modifying notebooks
- Saving changes
- Cloud synchronization
- Network connectivity
- Password-protected notebooks

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Troubleshooting

### Common Issues

**"Failed to parse notebook"**
- Ensure the file is a valid `.one` or `.onetoc2` format
- Legacy formats (OneNote 2010 and earlier) may have limited support
- Encrypted notebooks are not supported

**Application won't start**
- Verify GTK4 and libadwaita are installed: `sudo apt install libgtk-4-dev libadwaita-1-dev`
- Check for missing dependencies: `ldd target/release/onenote-viewer`

**Images not displaying**
- Some image formats may not be supported by GDK-Pixbuf
- Try converting images to PNG or JPEG in the original notebook

## Acknowledgments

- Thanks to the [`onenote_parser`](https://crates.io/crates/onenote_parser) crate authors
- Built with [`gtk4-rs`](https://gtk-rs.org/) bindings
- Inspired by the need for Linux-compatible OneNote viewers
