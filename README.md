# Spreaddit

A decentralized, Reddit-style link board built with **Stellar Soroban** smart contracts and a **React** frontend.

Spreaddit lets users submit links, add short descriptions, and vote links up or down. Contract state is stored on Soroban, while the frontend provides a lightweight interface for reading and updating that state through Soroban RPC.

## Features

- **On-chain link board**: Article URIs, descriptions, scores, and timestamps stored in Soroban contract state.
- **Upvote/downvote flow**: Vote on existing links or seed new submissions with an initial score.
- **Search and sorting**: Filter links by keyword and sort by top, newest, or oldest.
- **Configurable RPC**: Point the UI at a custom Soroban RPC endpoint.
- **Feed status visibility**: Result counts, timestamps, and load-error feedback in the UI.
- **Contract helpers**: Query a single article or total article count from the contract.

## Project Structure

```text
.
├── src/                    # Soroban smart contract (Rust)
│   ├── lib.rs
│   └── test.rs
├── spreddit-app/           # React frontend
│   ├── public/
│   └── src/
├── docs/                   # Project documentation
├── .github/workflows/      # CI workflow
├── Cargo.toml
└── README.md
```

## Tech Stack

- **Smart Contract**: Rust, Soroban SDK
- **Frontend**: React, Create React App, soroban-client
- **Tooling**: Cargo, npm, GitHub Actions

## Getting Started

### Prerequisites

- Rust toolchain (`rustup`, `cargo`)
- Node.js 20+ and npm
- (Optional) Soroban CLI and local Soroban RPC for contract deployment/testing

### 1) Clone the repository

```bash
git clone https://github.com/authdiff-labs/Spreaddit.git
cd Spreaddit
```

### 2) Run the frontend

```bash
cd spreddit-app
npm install
npm start
```

Open `http://localhost:3000`.

### 3) Configure environment (optional)

Create `spreddit-app/.env` from `spreddit-app/.env.example`:

```bash
REACT_APP_SOROBAN_RPC_URL=https://futurenet.sorobandev.com/soroban/rpc
REACT_APP_SPREADDIT_CONTRACT_ID=3d86cd584daedcf04c37ea5546a6689637b1fd75bd8bd3f89cbc61bbd37d9302
REACT_APP_NETWORK_PASSPHRASE=Test SDF Future Network ; October 2022
```

## Smart Contract

### Contract API

- `vote(uri: Bytes, amount: i32, descr: Bytes) -> State`
  - Submits a new article when unseen and `amount > 0`
  - Updates score for existing articles
  - Removes article when score becomes `<= 0`
  - Emits `SUBMIT`, `UPVOTE`, or `DOWNVOTE` events
- `refr_state() -> State` – returns current contract state
- `get_article(uri: Bytes) -> Option<ArticleData>` – returns a single article
- `article_count() -> u32` – returns number of tracked articles

### Local contract checks

```bash
cargo check
cargo test
```

> Note: This repository currently pins legacy Soroban dependencies from the original prototype. Some environments may require dependency updates or compatible toolchains to build/test successfully.

## Frontend Usage

1. Set a Soroban RPC endpoint (or use default).
2. Enter a Futurenet secret key (for write actions).
3. Post a link with an initial vote amount and description.
4. Vote on links from the feed.
5. Use search/sort controls to navigate content.

## Development

### Useful commands

```bash
# Contract
cargo check
cargo test

# Frontend
cd spreddit-app
npm install
npm start
npm run build
```

### CI

GitHub Actions workflow (`.github/workflows/ci.yml`) runs:
- Rust `cargo check`
- Frontend `npm ci` and build in `spreddit-app`

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Contract API](docs/API.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

## Roadmap

- [ ] Integrate Stellar Asset Contract / token transfer flow for paid voting
- [ ] Migrate to current Soroban SDK and soroban-client versions
- [ ] Add event-based feed updates instead of polling
- [ ] Add wallet integration (Freighter or similar) instead of manual secret key input
- [ ] Improve test coverage and add end-to-end UI tests
- [ ] Add deployment scripts and environment-specific config

## Security

Please do not report vulnerabilities in public issues. See [SECURITY.md](SECURITY.md).

## License

MIT. See [LICENSE](LICENSE).
