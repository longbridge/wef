# Contributing to Wef

Thank you for your interest in contributing to Wef! This document provides guidelines and information for contributors.

## Prerequisites

Before you can build and test Wef, you'll need:

### System Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y libglib2.0-dev pkg-config build-essential
```

#### Linux (CentOS/RHEL/Fedora)  
```bash
sudo yum install -y glib2-devel pkgconfig gcc-c++
# or for newer versions:
sudo dnf install -y glib2-devel pkgconfig gcc-c++
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Rust Toolchain

Install Rust using [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/longbridge/wef.git
   cd wef
   ```

2. **Install cargo-wef**
   ```bash
   cargo install --path cargo-wef
   ```

3. **Initialize CEF**
   ```bash
   cargo wef init
   ```
   This downloads the Chromium Embedded Framework binaries to `~/.cef`

4. **Build the project**
   ```bash
   cargo wef build
   ```

5. **Run tests**
   ```bash
   cargo test --all
   ```

## Project Structure

- **`wef/`** - Core library implementing CEF3 bindings
- **`cargo-wef/`** - Command-line tool for building wef applications
- **`examples/`** - Example applications demonstrating usage

## Building and Testing

### Building
```bash
# Build everything
cargo wef build

# Build specific components
cargo build -p wef           # Core library
cargo build -p cargo-wef     # CLI tool  
```

### Testing
```bash
# Run all tests
cargo test --all

# Test specific components  
cargo test -p wef
cargo test -p cargo-wef
```

### Running Examples
```bash
# Run the winit example
cargo wef run --example wef-winit
```

## Code Style

We use the standard Rust formatting tools:

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all -- -D warnings
```

## Submitting Changes

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`
3. **Make your changes** following the coding standards
4. **Add tests** for new functionality
5. **Ensure all tests pass** with `cargo test --all`  
6. **Format your code** with `cargo fmt --all`
7. **Run clippy** with `cargo clippy --all -- -D warnings`
8. **Commit your changes** with clear, descriptive commit messages
9. **Push to your fork** and create a Pull Request

## Commit Messages

Use clear, descriptive commit messages following conventional commits:

- `feat: add new functionality`
- `fix: resolve bug in component`  
- `docs: update README`
- `refactor: improve code structure`
- `test: add test coverage`

## Pull Request Guidelines

- **Describe your changes** clearly in the PR description
- **Reference related issues** using keywords like "Fixes #123"
- **Keep changes focused** - one feature or fix per PR
- **Update documentation** if you're changing public APIs
- **Add tests** for new functionality
- **Ensure CI passes** before requesting review

## Issues and Feature Requests

- Check existing issues before creating new ones
- Use clear, descriptive titles
- Provide reproduction steps for bugs
- Include system information (OS, Rust version, etc.)

## Development Tips

### CEF Troubleshooting

If you encounter CEF-related build issues:

1. **Clear CEF cache**
   ```bash
   rm -rf ~/.cef
   cargo wef init
   ```

2. **Check CEF version**
   The CEF version is defined in `.github/workflows/ci.yml`

3. **Verify system dependencies**
   Ensure all required system packages are installed

### Debugging

- Use `cargo wef run --example wef-winit` to test changes quickly
- Enable debug logging with `RUST_LOG=debug`
- Use the debugger-friendly debug profile for development

## Getting Help

- **Documentation**: Check the [README](README.md) and [library docs](wef/README.md)
- **Issues**: Search existing [GitHub issues](https://github.com/longbridge/wef/issues)
- **Discussions**: Use [GitHub Discussions](https://github.com/longbridge/wef/discussions) for questions

## License

By contributing to Wef, you agree that your contributions will be licensed under the Apache License 2.0.