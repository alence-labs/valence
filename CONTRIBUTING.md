# Contributing to Valence

Thank you for your interest in contributing to Valence! This project is focused on building open source tooling and smart contract infrastructure for Stellar Soroban.

## Getting started

1. Fork the repository.
2. Create a branch for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes.
4. Run tests and validate the contract build locally.

## Project structure

- `contracts/valence-core` — Soroban contract implementation
- `backend` — Axum backend service
- `frontend` — Vite + React UI scaffold
- `crates/valence-types` — shared Rust types and models

## Build and test

### Rust workspace

```bash
cargo test --workspace
cargo build --manifest-path contracts/valence-core/Cargo.toml --target wasm32v1-none --release
```

### Soroban contract build

```bash
stellar contract build --manifest-path contracts/valence-core/Cargo.toml
```

### Frontend

```bash
cd frontend
npm install
npm run build
```

## Contribution workflow

- Open an issue first for larger enhancements or architectural changes.
- Keep pull requests small and focused.
- Include tests or reproduction steps when possible.
- Update documentation when adding new features or changing behavior.

## Coding style

- Keep Rust code idiomatic and compatible with Soroban contract requirements.
- Avoid `std` dependencies in contract code.
- Follow existing workspace feature conventions.

## Pull requests

- Base your PR against `main`.
- Include a clear summary of the change.
- Mention any build or deployment steps you verified.

## Reporting issues

If you find a bug or want to suggest an improvement, please open an issue with:

- A short summary of the problem
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project.
