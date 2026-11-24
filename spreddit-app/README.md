# Spreaddit Frontend

React frontend for the Spreaddit Soroban dApp.

## Scripts

```bash
npm install
npm start
npm test
npm run build
```

## Configuration

Copy `.env.example` to `.env` and adjust values as needed:

- `REACT_APP_SOROBAN_RPC_URL`
- `REACT_APP_SPREADDIT_CONTRACT_ID`
- `REACT_APP_NETWORK_PASSPHRASE`

## Notes

- The app uses `soroban-client` to read contract state and submit transactions.
- Write actions require a Futurenet secret key in the UI (development/demo flow).
