# FavKit

[![Build Status](https://github.com/screwyprof/favkit/actions/workflows/rust.yml/badge.svg)](https://github.com/screwyprof/favkit/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/screwyprof/favkit/graph/badge.svg?token=B5ARXL56RN)](https://codecov.io/gh/screwyprof/favkit)
[![Minimum Rust Version](https://img.shields.io/badge/MSRV-nightly-red)](https://github.com/rust-lang/api-guidelines/discussions/231)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A modern Rust library and CLI tool for managing macOS Finder favorites. This project is a modern replacement for the abandoned [mysides](https://github.com/mosen/mysides) tool.

## Installation

1. Download the latest release from the [releases page](https://github.com/screwyprof/favkit/releases)
2. Unzip the downloaded file:
   ```bash
   unzip favkit-macos.zip
   ```
3. Remove quarantine:
   ```bash
   sudo xattr -d com.apple.quarantine favkit
   ```
4. Move to a directory in your PATH (optional):
   ```bash
   sudo mv favkit /usr/local/bin/
   ```

## Project Goals

1. **Primary Goal**: Create a maintained alternative to `mysides` for managing macOS Finder favorites
2. **Learning**: Serve as a Rust programming kata exploring:
   - Clean Architecture principles
   - Outside-In TDD (London School)
   - Domain-Driven Design
   - Modern development practices with nix + direnv

## Status: 🚧 Under Development

This project is currently in alpha stage. Progress and next steps:

✅ **Completed**:
- Basic viewing of Finder favorites
- Proper display names for special locations:
  - AirDrop: Shows as "AirDrop" without exposing internal URL
  - Recents: Shows as "Recents" without exposing internal URL
  - Applications: Shows as "Applications" without exposing internal URL

🚧 **In Progress**:
- User-friendly path formatting (show regular paths instead of raw URLs)

🔜 **Planned**:
- Handle special locations:
  - User Desktop (`file:///Users/<user>/Desktop/`)
  - User Downloads (`file:///Users/<user>/Downloads/`)
- Add/remove favorites
- Command-line interface improvements

## Documentation

- [Architecture Overview](docs/architecture.md)
- [Requirements](docs/requirements.md)
- [Development Guide](docs/development.md)
- [Architecture Decisions](docs/adr/)

## Development

The project uses nix + direnv for reproducible development environment:

```bash
# Setup development environment
direnv allow

# See all available commands
make
# or
make help

# Run development tools
make all # run linters, tests and build project
```

## Contributing

See our [Contributing Guide](CONTRIBUTING.md) for details on how to get involved with the project.

## License

[MIT](LICENSE)
