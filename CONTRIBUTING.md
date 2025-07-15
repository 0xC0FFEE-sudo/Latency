# Contributing to Latency-X

Thank you for your interest in contributing to Latency-X! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Node.js 18 or later
- Git
- Basic understanding of high-frequency trading concepts

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/latency-x.git
   cd latency-x
   ```

3. Set up the development environment:
   ```bash
   # Backend setup
   cd latency-x-core
   cargo build
   
   # Frontend setup
   cd ../latency-x-dashboard
   npm install
   ```

4. Copy the example configuration:
   ```bash
   cp latency-x-core/Config.toml.example latency-x-core/Config.toml
   ```

## 🛠️ Development Guidelines

### Code Style

#### Rust Code
- Follow the official Rust style guide
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Write comprehensive tests for new functionality

#### TypeScript/React Code
- Use TypeScript for all new code
- Follow React best practices and hooks patterns
- Use Tailwind CSS for styling
- Ensure components are accessible

### Testing

- Write unit tests for all new functionality
- Integration tests for API endpoints
- Frontend tests for critical user interactions
- Run the full test suite before submitting PRs:

```bash
# Backend tests
cd latency-x-core
cargo test

# Frontend tests
cd latency-x-dashboard
npm test
```

### Documentation

- Update README.md for significant changes
- Add inline documentation for complex algorithms
- Update API documentation for new endpoints
- Include examples for new features

## 📝 Contribution Process

### 1. Issue Creation

Before starting work:
- Check existing issues to avoid duplication
- Create an issue describing the problem or feature
- Wait for maintainer feedback on approach

### 2. Branch Naming

Use descriptive branch names:
- `feature/add-new-strategy`
- `fix/websocket-connection-issue`
- `docs/update-api-documentation`

### 3. Commit Messages

Follow conventional commit format:
```
type(scope): description

feat(strategies): add momentum trading strategy
fix(dashboard): resolve WebSocket reconnection issue
docs(readme): update installation instructions
```

### 4. Pull Request Process

1. Ensure your branch is up to date with main
2. Run all tests and ensure they pass
3. Update documentation as needed
4. Create a detailed PR description including:
   - What changes were made
   - Why the changes were necessary
   - How to test the changes
   - Screenshots for UI changes

### 5. Code Review

- Address all reviewer feedback
- Keep discussions professional and constructive
- Be open to suggestions and alternative approaches

## 🎯 Areas for Contribution

### High Priority
- **New Trading Strategies**: Implement additional algorithmic trading strategies
- **Exchange Connectors**: Add support for new cryptocurrency exchanges
- **Performance Optimization**: Improve latency and throughput
- **Risk Management**: Enhance risk controls and position management

### Medium Priority
- **Dashboard Features**: Add new visualization and monitoring capabilities
- **Backtesting**: Improve historical data testing framework
- **Documentation**: Expand guides and API documentation
- **Testing**: Increase test coverage

### Low Priority
- **UI/UX Improvements**: Polish the dashboard interface
- **Configuration**: Simplify setup and configuration process
- **Logging**: Enhance logging and debugging capabilities

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment Information**:
   - Operating system
   - Rust version
   - Node.js version

2. **Steps to Reproduce**:
   - Clear, numbered steps
   - Expected vs actual behavior
   - Screenshots if applicable

3. **Additional Context**:
   - Log files (sanitized of sensitive data)
   - Configuration details
   - Error messages

## 💡 Feature Requests

For new features:

1. **Use Case**: Describe the problem you're trying to solve
2. **Proposed Solution**: Your idea for implementation
3. **Alternatives**: Other approaches you've considered
4. **Impact**: Who would benefit from this feature

## 🔒 Security

- Never commit API keys, passwords, or sensitive data
- Report security vulnerabilities privately to maintainers
- Use environment variables for sensitive configuration
- Follow secure coding practices

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Discord**: [Join our community](https://discord.gg/latency-x) (if available)

## 🏆 Recognition

Contributors will be:
- Listed in the project's contributors section
- Mentioned in release notes for significant contributions
- Invited to join the core team for exceptional contributions

## 📋 Checklist

Before submitting a PR, ensure:

- [ ] Code follows project style guidelines
- [ ] Tests pass locally
- [ ] Documentation is updated
- [ ] Commit messages follow convention
- [ ] PR description is complete
- [ ] No sensitive data is included

Thank you for contributing to Latency-X! 🚀