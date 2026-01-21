# Contributing to RepoCard Studio

Thank you for your interest in contributing! This is a free, community-driven tool.

## How to Contribute

### Reporting Bugs

1. Check existing issues first
2. Open a new issue with:
   - Clear description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - OS and app version

### Suggesting Features

1. Open a discussion or issue
2. Describe the use case
3. Keep scope minimal — this is a focused utility

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `npm test` and `cargo test`
5. Submit a PR with a clear description

## Development Setup

```bash
# Prerequisites: Node.js 18+, Rust stable, Tauri prerequisites

# Clone and install
git clone https://github.com/Brutus1066/Repocard-Studio-KZ.git
cd Repocard-Studio-KZ/apps/repocard-studio
npm install

# Run in development
npm run tauri dev

# Run tests
npm test
cd src-tauri && cargo test
```

## Code Style

- TypeScript: Follow existing patterns, use `npm run lint`
- Rust: Use `cargo fmt` and `cargo clippy`
- Keep commits focused and messages clear

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Questions? Open an issue or start a discussion.
