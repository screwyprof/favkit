# Development Guide

## Getting Started

### Prerequisites
- Nix package manager
- nix-direnv

### Setup
1. Clone the repository
2. Run `direnv allow` (this will set up all required development tools via flake.nix)

## Development Workflow

### 1. ATDD Cycle
1. Write acceptance test in `tests/acceptance.rs`
2. Run tests (`make test`)
3. Implement feature
4. Refactor
5. Document changes

### 2. Code Quality
```bash
make fmt      # Format code
make lint     # Run clippy
make check    # Type checking
```

### 3. Coverage
```bash
make coverage         # Full HTML report
make coverage-text    # Quick terminal summary
```

## Documentation

### Architecture
- Check `docs/architecture.md` for system overview
- Review ADRs in `docs/adr/` for design decisions

### Code
- Use doc comments (`///`) for public APIs
- Include examples in doc tests
- Update README.md for user-facing changes

## Release Process
1. Update version in `Cargo.toml`
2. Update changelog
3. Create git tag
4. Build release binary
5. Update documentation
