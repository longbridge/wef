# Wef - Web Embedding Framework

![CI](https://github.com/longbridge/wef/workflows/CI/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/wef.svg)](https://crates.io/crates/wef)
[![Documentation](https://docs.rs/wef/badge.svg)](https://docs.rs/wef)

**Wef** (Web Embedding Framework) is a Rust library for embedding WebView functionality using Chromium Embedded Framework (CEF3) with offscreen rendering support.

> The `Wef` name is an abbreviation of "Web Embedding Framework", and it's also inspired by Wry.

![Wef Example](https://github.com/user-attachments/assets/f677ecb4-dbff-4e0d-86b9-203f6e1004a4)

## Features

- **Cross-Platform**: Support for Windows, macOS, and Linux
- **CEF3 Integration**: Built on top of Chromium Embedded Framework for reliable web rendering
- **Offscreen Rendering**: Advanced rendering capabilities with offscreen support
- **JavaScript Bridge**: Seamless communication between Rust and JavaScript
- **Multi-Process Architecture**: Leverages CEF's multi-process design for stability
- **Cargo Integration**: Complete toolchain with `cargo-wef` for easy development

## Quick Start

### Installation

Add `wef` to your `Cargo.toml`:

```toml
[dependencies]
wef = "0.7.0"
```

### Install cargo-wef

```bash
cargo install cargo-wef
```

### Initialize CEF

```bash
cargo wef init
```

### Basic Usage

```rust
use wef::Settings;

fn main() {
    let settings = Settings::new();
    wef::launch(settings, || {
        // Your application logic here
    });
}
```

## Documentation

For comprehensive documentation, examples, and API reference, see the library documentation within this repository.

## Project Structure

This repository contains:

- **Core Library** - The main wef library (in the root directory)
- **[`tool/`](tool/)** - Command-line tool (`cargo-wef`) for building and managing wef applications  
- **[`examples/`](examples/)** - Example applications demonstrating wef usage

## Development

### Building

```bash
# Build the library
cargo wef build

# Run tests
cargo test --all

# Run an example
cargo wef run --example wef-winit
```

### Requirements

- CEF binary distribution (automatically downloaded by `cargo-wef`)
- Platform-specific dependencies:
  - **Linux**: `libglib2.0-dev`, `pkg-config`
  - **Windows**: Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE-APACHE](LICENSE-APACHE) for details.