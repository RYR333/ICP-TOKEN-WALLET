# ICP Token Wallet

This project is a secure token wallet built on the Internet Computer (ICP) blockchain. It supports IRCRC2 token transactions and smart contract deployment on a local ICP test network.

## Features
- IRCRC2 token transactions
- Smart contract deployment on the ICP testnet
- Secure token storage
- User authentication

## Prerequisites
Ensure you have the following installed:
- [DFX SDK](https://smartcontracts.org/docs/developers-guide/install-upgrade-remove.html)
- Rust & Cargo
- Node.js & npm (for frontend development)

## Setup
### 1. Clone the repository
```bash
 git clone <repository-url>
 cd icp_token_wallet
```

### 2. Start the local ICP environment
```bash
dfx start --background
```

### 3. Create and deploy canisters
```bash
dfx canister create wallet_canister_backend
```
```bash
dfx build wallet_canister_backend
```
```bash
dfx deploy
```

## Accessing the Wallet
- **Frontend UI:** [http://localhost:4943/](http://localhost:4943/)
- **Candid Interface (Backend API):** [http://127.0.0.1:4943/?canisterId=<backend-canister-id>](http://127.0.0.1:4943/?canisterId=<backend-canister-id>)

## Commands
### Check Canister Status
```bash
dfx canister status wallet_canister_backend
```
```bash
dfx canister status wallet_canister_frontend
```

### View Logs
```bash
dfx canister log wallet_canister_backend
```

## Deployment on Mainnet
If deploying to the ICP mainnet, ensure you have enough cycles and an authorized identity:
```bash
dfx identity new my_identity
```
```bash
dfx identity use my_identity
```
```bash
dfx deploy --network ic
```

## Troubleshooting
### Reset Local Environment
```bash
dfx stop
rm -rf .dfx
```
Then restart:
```bash
dfx start --background
```

## License
MIT License

