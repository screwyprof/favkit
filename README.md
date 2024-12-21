# FavKit

[![codecov](https://codecov.io/gh/screwyprof/favkit/graph/badge.svg?token=B5ARXL56RN)](https://codecov.io/gh/screwyprof/favkit)

A modern Rust library and CLI tool for managing macOS Finder favorites. This project is a modern replacement for the abandoned [mysides](https://github.com/mosen/mysides) tool.

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

🚧 **In Progress**:
- Improve display names (show "AirDrop" instead of "<no name>")
- User-friendly path formatting (show regular paths instead of raw URLs)

🔜 **Planned**:
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
make test

# Run development tools
make # run linters, tests and build project
```

## License

MIT

## Contributing

See our [Contributing Guide](CONTRIBUTING.md) for details on how to get involved with the project.