
# OneNote Viewer

[![CI](https://github.com/yourusername/onenote-viewer/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/onenote-viewer/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast, native, **read-only** desktop viewer for Microsoft OneNote `*.one` notebook files on Linux.

> ⚠️ **This is an unofficial app!** It is not affiliated with, endorsed by, or connected to Microsoft Corporation. OneNote is a trademark of Microsoft Corporation.

## ✨ Features

- 📂 **Open Notebooks**: Load `.onetoc2` notebook roots or standalone `.one` files
- 🌳 **Hierarchical Sidebar**: Navigate Notebook → Sections → Pages
- 📄 **Rich Content Rendering**:
  - Text with basic formatting (bold, italic, lists, headers)
  - Embedded images displayed inline
  - Attachments with icons, filenames, and sizes (click to open)
- 🎨 **Modern UI**: Built with GTK4 and libadwaita for a native GNOME experience
- 🌓 **Theme Support**: Automatically follows system light/dark theme
- ⚡ **Async Parsing**: Non-blocking background parsing for smooth UX
- 🛡️ **Graceful Degradation**: Skips unsupported content (ink, equations, encrypted pages) with user-friendly warnings
- 📦 **Debian Ready**: Pre-built `.deb` packages for Debian/Ubuntu (amd64, arm64)

## 🖼️ Screenshots

*(Add screenshots here once available)*

## 📥 Installation

### From .deb Package (Recommended)

Download the latest `.deb` package from the [Releases](https://github.com/yourusername/onenote-viewer/releases) page and install:

```bash
sudo apt install ./onenote-viewer_*.deb
```

### Build from Source

#### Prerequisites

Ensure you have the following installed:

- Rust (1.70+ recommended)
- GTK4 and libadwaita development libraries
- Additional build dependencies

**On Debian/Ubuntu:**

```bash
sudo apt update
sudo apt install -y \
    curl \
    build-essential \
    libgtk-4-dev \
    libadwaita-1-dev \
    pkg-config \
    cargo-deb
```

**Install Rust (if not already installed):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Build Instructions

```bash
# Clone the repository
git clone https://github.com/yourusername/onenote-viewer.git
cd onenote-viewer

# Build in release mode
cargo build --release

# Run the application
cargo run --release
```

#### Create .deb Package

```bash
cargo deb
```

The generated package will be in `target/debian/`.

## 🚀 Usage

### Launch the Application

```bash
onenote-viewer
```

### Open a File via CLI

```bash
onenote-viewer /path/to/notebook.onetoc2
# or
onenote-viewer /path/to/page.one
```

### Opening Files

1. Click the **"Open Notebook"** button or use `Ctrl+O`
2. Navigate to your `.onetoc2` or `.one` file
3. Browse the sidebar hierarchy and view page content

## 📁 Supported File Formats

| Format | Description | Support Level |
|--------|-------------|---------------|
| `.onetoc2` | OneNote Notebook Table of Contents | ✅ Full |
| `.one` | OneNote Section/Page File | ✅ Full |
| Legacy FSSHTTP (2010) | Older OneNote format | ⚠️ Partial (may skip some content) |
| Encrypted/Password-protected | Password-locked notebooks | ❌ Not supported |

## ⚠️ Limitations

As a read-only viewer, this application does **not** support:

- Editing or modifying notebooks
- Saving changes
- Syncing with OneDrive or OneNote Online
- Rendering complex ink drawings or handwritten notes
- Solving mathematical equations
- Opening password-protected or encrypted notebooks

Unsupported content is gracefully skipped with a non-blocking warning notification.

## 🛠️ Technology Stack

- **Language**: Rust (2021 Edition)
- **GUI Framework**: GTK4 + libadwaita (`gtk4-rs`)
- **Parser**: [`onenote_parser`](https://crates.io/crates/onenote_parser)
- **Async Runtime**: Tokio + GLib main context
- **Packaging**: `cargo-deb`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues, fork the repository, and send pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code passes `cargo fmt` and `cargo clippy` before submitting.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- Thanks to the [`onenote_parser`](https://crates.io/crates/onenote_parser) crate authors for the parsing library
- Thanks to the [`gtk4-rs`](https://gtk-rs.org/) community for excellent GTK4 bindings
- Inspired by the need for a native Linux OneNote viewer

## 📬 Contact

- **Issues**: [GitHub Issues](https://github.com/yourusername/onenote-viewer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/onenote-viewer/discussions)

---

*Built with ❤️ for the Linux community*
