# Contributing to LuminaRWA Protocol

## Code of Conduct

Be respectful, inclusive, and professional in all interactions.

## How to Contribute

### Reporting Issues

1. Check existing issues first
2. Provide clear, reproducible steps
3. Include environment details (Rust version, OS, etc.)
4. Include error messages and logs

### Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/description`
3. Make your changes
4. Follow the code style guidelines
5. Add/update tests
6. Commit with clear messages: `git commit -m "[Task N] Description"`
7. Push and create a pull request

## Code Style

### Rust Standards

- Follow standard Rust naming conventions
- Use `cargo fmt` for formatting
- Pass `cargo clippy` without warnings
- Use meaningful variable names
- Add documentation comments for public items

### Commit Messages

```
[Task N] Brief description (50 chars max)

Longer explanation if needed. Include:
- What changed
- Why it changed
- How it was tested

Closes #issue-number (if applicable)
```

### Branch Naming

```
feature/feature-name      # New feature
fix/bug-name             # Bug fix
docs/documentation-name  # Documentation
refactor/component-name  # Code refactor
```

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure all tests pass: `cargo test`
4. Ensure code formatting: `cargo fmt`
5. Ensure no clippy warnings: `cargo clippy`
6. Write clear PR description
7. Link related issues

## Testing Requirements

- Minimum 95% code coverage
- All error paths tested
- Edge cases covered
- Integration tests for new features

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Coverage report
cargo tarpaulin --out Html
```

## Documentation Requirements

- Public functions must have doc comments
- Include examples in doc comments where helpful
- Update README.md if adding features
- Update specs/tasks.md if modifying requirements

## Security Considerations

- Never commit secrets or private keys
- Use `.gitignore` for sensitive files
- Report security issues privately
- Follow secure coding practices

## Performance Guidelines

- Profile before optimizing
- Document performance changes
- Include benchmarks for critical paths
- Target <100ms for transfer verification

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create release branch
4. Tag release: `git tag v1.0.0`
5. Push tag: `git push origin v1.0.0`
6. Create GitHub release with notes

## Development Workflow

### Local Setup

```bash
git clone https://github.com/drips-network/luminarwa-protocol.git
cd luminarwa-protocol
cargo build
cargo test
```

### Before Committing

```bash
cargo fmt              # Format code
cargo clippy           # Lint
cargo test             # Test
cargo build --target wasm32-unknown-unknown --release
```

### Continuous Integration

GitHub Actions runs:
- Unit tests
- Code formatting checks
- Clippy linting
- WASM build verification

All must pass before merging.

## Questions?

- Check existing documentation
- Review past PRs for similar work
- Ask in issues or discussions
- Review specs/luminarwa.md for design decisions

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to LuminaRWA Protocol!
