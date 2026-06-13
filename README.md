# broker

A custodial Bitcoin broker backend written in Rust. It exposes an HTTP API for user
accounts, KYC onboarding, encrypted custodial wallets and on-chain transactions, backed
by MySQL and a Bitcoin Core node.

> ⚠️ **Educational / portfolio project.** It implements real custody and cryptography
> patterns but has not been audited. Do not use it with real funds.

## Features

- **Authentication** — registration and login with password hashing (Argon2 / bcrypt)
  and stateless JWT sessions.
- **Custodial wallets** — BIP39 mnemonic generation with per-user entropy; private key
  material is encrypted at rest with AES-256-GCM under a key-encryption-key (KEK)
  derived from a master key, and zeroized in memory after use.
- **KYC** — multi-step identity verification flow with document upload (validated via
  MIME sniffing and filename sanitization).
- **Transactions** — on-chain transfers issued through a Bitcoin Core RPC client.
- **Admin** — administrative endpoints for user management.

## Tech stack

| Layer        | Technology                                   |
|--------------|----------------------------------------------|
| Language     | Rust (edition 2024)                          |
| HTTP         | Actix-web 4 (CORS, multipart, static files)  |
| Async        | Tokio                                        |
| Database     | MySQL via SQLx                               |
| Auth         | JWT (`jsonwebtoken`), Argon2 + bcrypt        |
| Crypto       | `bitcoin`, `bip39`, `aes-gcm`, `zeroize`     |
| Node RPC     | `corepc` (Bitcoin Core)                      |
| Packaging    | Docker + Docker Compose                      |

## Getting started

The setup order matters:

```sh
# 1. Generate the master key used to derive wallet KEKs
./generate_master_key.sh

# 2. Build and start the API, database and init services
docker compose build
docker compose up
```

The `init/` service creates the database schema (`init/init.sql`) and bootstrap data.

## Project layout

```
src/
├── main.rs              # Actix-web app & route wiring
├── jwt.rs               # JWT issuing / validation
├── json.rs, enums.rs    # API types & shared enums
├── env.rs               # configuration from environment
├── admin/               # admin endpoints (user management)
└── impl_user/
    ├── basic_fn.rs      # core user operations
    ├── crypto/          # wallet generation, entropy, KEK derivation
    ├── kyc/             # KYC creation & verification flow
    └── rpc/             # Bitcoin Core transactions
init/                    # DB schema & bootstrap (Dockerized)
```

## License

Released under the [MIT License](LICENSE).
