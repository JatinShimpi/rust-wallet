# ICP Rust Token Wallet

A secure Rust-based token wallet for the Internet Computer Protocol (ICP) blockchain, supporting IRCRC2 token transfers, balance queries, and basic security features. Built for deployment on ICP testnets.

---

## 📌 Overview
This project provides a minimal, self-contained token wallet implementation on the ICP blockchain, including:
- Smart contracts for token transfers and balance tracking.
- A CLI for interacting with the wallet.
- Security features like principal authorization.

---

## 🚀 Features
- **Send/Receive Tokens**: Transfer IRCRC2 tokens between principals.
- **Balance Queries**: Fetch real-time token balances.
- **Authorization Checks**: Whitelist-based security for authorized users.
- **Local/Testnet Deployment**: Deploy to a local replica or ICP testnet.
- **Unit Tests**: Pre-built tests for core functionalities.

---

## 📋 Prerequisites
1. **Rust**: Install via [rustup](https://rustup.rs/).
2. **DFX SDK**: Follow [ICP's installation guide](https://smartcontracts.org/docs/developers-guide/install.html).
3. **Node.js**: For optional frontend interactions (v14+).
4. **IC Agent**: Included in `Cargo.toml`.

---

## ⚙️ Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/icp-token-wallet.git
   cd icp-token-wallet

2. Install dependencies:
   ```bash
   dfx start --background  # Start local ICP replica
   cargo build

## 🛠️ Usage
Deploy the Canister
    ```bash
    dfx deploy --network ic_testnet wallet_backend

CLI Commands
1. Send Tokens:
    ```bash
    cargo run -- send <PRINCIPAL_ID> 100

    Example:
    cargo run -- send 6sgbu-nyaaa-aaaah-qcxga-cai 500

2. Check Balance:

    ```bash
    cargo run -- balance <PRINCIPAL_ID>

    Example:
    cargo run -- balance 6sgbu-nyaaa-aaaah-qcxga-cai

## 🧪 Testing
Run unit tests for the smart contract:

    cargo test -- --test-threads=1

## 🔒 Security
1. Authorized Users: Modify src/wallet_backend/security.rs to whitelist principals.
2. Agent Configuration: The CLI uses http://localhost:8000 by default. Change this in wallet_cli/main.rs for testnet:
   ```rust
   .with_url("https://ic0.app")

## 🌐 Deployment to ICP Testnet
1. Ensure DFX is authenticated:
   ```bash
   dfx identity new dev --storage-mode=plaintext
   dfx identity use dev

2. Deploy:
   ```bash
   dfx deploy --network ic_testnet wallet_backend
