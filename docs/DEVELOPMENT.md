# Development Guide

## Local Setup

### Contract
- Install Rust stable toolchain.
- Run `cargo check` from repo root.
- Run `cargo test` for contract unit tests.

### Frontend
- Install Node.js 20+.
- From `spreddit-app/`, run `npm install` and `npm start`.

## Environment Variables

Frontend supports:

- `REACT_APP_SOROBAN_RPC_URL`
- `REACT_APP_SPREADDIT_CONTRACT_ID`
- `REACT_APP_NETWORK_PASSPHRASE`

See `.env.example` and `spreddit-app/.env.example`.

## Common Tasks

- Update contract logic: `src/lib.rs`
- Update contract tests: `src/test.rs`
- Update UI behavior: `spreddit-app/src/App.js`
- Update docs: `README.md`, `docs/*.md`

## Known Gaps

- Legacy Soroban dependency versions may not build in all modern environments.
- Frontend write flow uses manual secret key entry (demo-only pattern).
