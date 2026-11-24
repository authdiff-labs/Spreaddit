# Contributing to Spreaddit

Thanks for your interest in contributing to Spreaddit.

## Getting Started

1. Fork the repository and clone your fork.
2. Create a feature branch from `main`.
3. Make focused, well-documented changes.
4. Run relevant checks before opening a pull request.

## Development Setup

### Smart Contract (Rust / Soroban)

```bash
cargo check
cargo test
```

### Frontend (React)

```bash
cd spreddit-app
npm install
npm start
```

## Pull Request Guidelines

- Keep PRs focused and descriptive.
- Include context on what changed and why.
- Update documentation when behavior or setup changes.
- Add or update tests when applicable.

## Commit Messages

Use clear, conventional-style commit messages, for example:

- `feat: add article filtering`
- `fix: handle invalid RPC URL`
- `docs: update setup instructions`

## Reporting Issues

Please include:

- Expected behavior
- Actual behavior
- Steps to reproduce
- Environment details (OS, Node/Rust versions)
