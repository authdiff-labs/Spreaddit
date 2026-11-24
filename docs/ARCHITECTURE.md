# Architecture

## Overview

Spreaddit is a decentralized Reddit-style link board built on Stellar Soroban.

- **Smart contract (`src/lib.rs`)**: stores article state and vote counts on-chain.
- **Frontend (`spreddit-app/`)**: React UI that reads contract state and submits votes via Soroban RPC.

## Contract Model

### `ArticleData`
- `uri`: article URL bytes
- `count`: net vote score
- `descr`: description bytes
- `created`: ledger timestamp at first submission
- `updated`: ledger timestamp at last vote

### `State`
- `articles`: map keyed by article URI bytes

### Contract Methods
- `vote(uri, amount, descr) -> State`
  - Creates a new article when `amount > 0` and URI is unseen
  - Updates score for existing articles
  - Removes article when score falls to `<= 0`
  - Emits `SUBMIT`, `UPVOTE`, or `DOWNVOTE` events
- `refr_state() -> State`
  - Returns persisted contract state

## Frontend Flow

1. User configures Soroban RPC endpoint and Futurenet secret key.
2. App polls contract `STATE` ledger entry and renders articles.
3. User posts or votes on a link.
4. App builds/simulates/submits Soroban transaction.
5. UI refreshes to reflect updated state.

## Known Limitations

- No on-chain asset transfer for vote payments yet.
- Polling-based refresh instead of event subscription.
- Minimal validation and error handling in UI.
- Contract and frontend use legacy Soroban SDK/client versions.
