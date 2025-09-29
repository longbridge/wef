# Contributing to Wef

Thank you for your interest in contributing to Wef! This document provides guidelines and information for contributors.

## Prerequisites

Before you can build and test Wef, you'll need:

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
cargo wef run -p wef-example
```

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

- Use `cargo wef run -p wef-winit` to test changes quickly
- Enable debug logging with `RUST_LOG=debug`
- Use the debugger-friendly debug profile for development

## Getting Help

- **Documentation**: Check the [README](README.md) and [library docs](wef/README.md)
- **Issues**: Search existing [GitHub issues](https://github.com/longbridge/wef/issues)
- **Discussions**: Use [GitHub Discussions](https://github.com/longbridge/wef/discussions) for questions

## License

By contributing to Wef, you agree that your contributions will be licensed under the Apache License 2.0.
