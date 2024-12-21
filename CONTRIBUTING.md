# Contributing to FavKit

Thank you for your interest in contributing to `FavKit`! This document provides guidelines and instructions for contributing to the project. Please read our [README.md](README.md) for an overview of the project.

## Development Environment

### Prerequisites
- macOS
- Nix package manager
- nix-direnv

### Setup
1. Fork the repository
2. Clone your fork: `git clone https://github.com/screwyprof/favkit.git`
3. Set up development environment: `direnv allow`
4. Run tests to verify setup: `make test`

## Development Process

### 1. Pick an Issue
- Check existing issues or create a new one
- Comment on the issue you'd like to work on
- Wait for assignment or approval

### 2. Development Workflow
1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Follow ATDD cycle:
   - Write acceptance test
   - Implement feature
   - Refactor
   - Document changes
3. Ensure all tests pass: `make test`
4. Check code quality: `make lint`
5. Update documentation if needed

### 3. Code Style
- Follow Rust style guidelines
- Document public APIs with doc comments
- Keep functions focused and small
- Follow project's architectural patterns

### 4. Testing
- Write meaningful tests
- Maintain high test coverage
- Use test-driven development approach

### 5. Documentation
- Update relevant documentation
- Add ADRs for significant decisions
- Include examples in doc comments
- Keep [README.md](README.md) up to date

## Pull Request Process

1. **Prepare Changes**
   - Ensure all tests pass
   - Run linters and formatters
   - Update documentation

2. **Submit PR**
   - Create detailed PR description
   - Reference related issues
   - Make sure CI is green.

3. **Review Process**
   - Address review comments
   - Keep PR focused and small
   - Maintain professional discourse

4. **Merge Requirements**
   - All tests passing
   - Code review approved
   - Documentation updated
   - No merge conflicts

## License

By contributing to `FavKit`, you agree that your contributions will be licensed under the [MIT License](LICENSE). 