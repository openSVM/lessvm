# Contributing to lessVM

We love your input! We want to make contributing to lessVM as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## Any contributions you make will be under the Apache 2.0 Software License

In short, when you submit code changes, your submissions are understood to be under the same [Apache 2.0 License](http://choosealicense.com/licenses/apache-2.0/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/yourusername/lessvm/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/yourusername/lessvm/issues/new); it's that easy!

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Development Guidelines

### Code Style

- Follow Rust standard formatting (use `rustfmt`)
- Use meaningful variable names
- Comment complex logic
- Keep functions small and focused
- Use Rust idioms and best practices

### Testing

- Write unit tests for all new code
- Include integration tests for new features
- Test edge cases and error conditions
- Use property-based testing where appropriate
- Ensure all tests pass before submitting PR

### Gas Optimization

- Consider gas costs in all operations
- Use zero-copy operations where possible
- Minimize memory allocations
- Profile gas usage in tests

### Security

- Follow security best practices
- Add bounds checking for all memory operations
- Validate all account accesses
- Handle all error cases
- Document security considerations

### Documentation

- Update README.md for new features
- Document all public functions
- Include examples in documentation
- Update gas costs table
- Document breaking changes

## Pull Request Process

1. Update the README.md with details of changes to the interface
2. Update the version numbers in Cargo.toml and README.md
3. Add your changes to CHANGELOG.md
4. The PR will be merged once you have the sign-off of two maintainers

## Community

- Join our [Discord](https://discord.gg/lessvm)
- Follow us on [Twitter](https://twitter.com/lessVM)
- Read our [Blog](https://lessvm.blog)

## License

By contributing, you agree that your contributions will be licensed under its Apache 2.0 License. 