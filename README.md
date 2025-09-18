# 🦁 Dummy Clients

**Dummy Clients** is a **full-feature educational Solana client**, built in Rust.
It's not a production validator — instead, it's a **learning-first implementation** of Solana's networking, ledger, PoH, consensus, runtime, and RPC layers.

Our mission: give developers the ability to **understand, build, and hack the Solana protocol itself** without needing datacenter-grade hardware.

---

## 🚀 Features

* **Gossip Networking** → discover validators, slots, and cluster state
* **Ledger Store & Replay** → download and verify blocks locally
* **Proof of History Verifier** → replay Solana's global clock
* **Consensus Simulator (Tower BFT)** → explore forks, voting, lockouts
* **Execution Runtime (Sealevel Lite)** → run transactions and programs
* **Custom RPC** → serve data directly from your local replayed ledger

All features are **real-world compatible** — same wire protocols, ledger format, and runtime behaviors as Solana Labs' validator — but simplified and modular for clarity.

---

## 🧭 Why Build This?

Most Solana developers focus on dApps and smart contracts. Very few touch the **protocol internals**. Dummy Clients closes that gap.

By building and using it, you'll:

* Learn **how validators work under the hood**
* Understand **networking, ledger, and consensus** at a deep level
* Gain skills to become a **protocol-level engineer** (not just a dApp dev)
* Contribute to the Solana ecosystem in **core infrastructure**

---

## 🛠️ Roadmap

### ✅ Phase 1 — Gossip Listener

* Connect to Solana gossip over QUIC/UDP
* Log validator identities, slot announcements, and cluster health
* CLI tools: `dc gossip peers`, `dc gossip slots`

### 🔜 Phase 2 — Ledger Downloader & Parser

* Fetch blocks from validators
* Store in a local database
* Explore accounts & transactions offline

### 🔜 Phase 3 — Proof of History Verifier

* Implement PoH replay in Rust
* Confirm time-ordering of transactions

### 🔜 Phase 4 — Consensus Simulator (Tower BFT Lite)

* Simulate validator votes and fork choice
* Visualize finality and lockouts

### 🔜 Phase 5 — RPC Surface

* Implement JSON-RPC for `getBlock`, `getTransaction`, etc.
* Optional: add **custom analytics** beyond Solana RPC

📚 *Why*: Makes the client useful to external tools/dapps, like QuickNode or Helius.

### 🔜 Phase 6 — Mini Validator Mode

* **Replay** transactions locally for validation
* **Forward** transactions to real validators (like a proxy)
* **Simulate** partial ledger sync

📚 *Why*: Bridges the gap between light client and full validator. Think of it as "training wheels" for Solana validation.

### 🔜 Phase 7 — Finality Proofs

* **Transaction Finality Proofs** → generate verifiable cryptographic proofs that a transaction is finalized in the Solana ledger.
* **Storage Finality Proofs** → export proofs of account state (e.g., balances, contract storage) tied to finalized slots.
* **Portable Proof Format** → structure proofs so they can be verified outside Solana (e.g., in smart contracts on Ethereum, Cosmos, or other chains).

📚 *Why*: Today, Solana doesn't expose raw transaction/state proofs in a portable way. Dummy Client will fill that gap, enabling **trustless bridges, zk-light clients, and cross-chain apps**.

### 🔮 Phase 8 — Advanced Experiments

* **zk-Light Client**: use zero-knowledge proofs to verify Solana state transitions trustlessly.
* **Cross-Client Testing**: compare Dummy Client proofs and replay with Solana Labs + Firedancer implementations.
* **Custom Modules**: add unique tooling (e.g., "most active programs in last 10k slots").

📚 *Why*: Pushes Dummy Client beyond education into **research-grade interoperability**.

---

## 🏆 Endgame

By completing this roadmap, Dummy Client evolves into a **full-fledged Solana light client** with validator DNA:

* Networking (Gossip)
* Ledger Sync (Blocks + Accounts)
* PoH Verification
* Consensus Simulation
* RPC Exposure
* Mini Validator Execution
* Transaction & Storage Finality Proofs
* zk-Light Client capabilities

---

## ⚡ Getting Started

### Prerequisites

* Rust (latest stable)
* Cargo
* Git

### Clone the repo

```bash
git clone https://github.com/victorchukwuemeka/dummy_clients.git
cd dummy_clients
```

### Build

```bash
cargo build
```

### Run (example: gossip listener)

```bash
cargo run --bin dc-gossip
```

---

## 📂 Repo Structure

```
/crates
  dc-gossip/        # Gossip networking
  dc-ledger/        # Blockstore & ledger parsing
  dc-poh/           # Proof of History verifier
  dc-consensus/     # Tower BFT simulator
  dc-runtime/       # Sealevel-lite execution engine
  dc-rpc/           # RPC surface
  dc-cli/           # CLI tools
configs/
docs/
examples/
```

---

## 🏆 Vision

Dummy Clients will become the **protocol gym** for Solana developers:

* A place to train, build, and experiment with validator-level concepts.
* A toolkit for teaching protocol engineering.
* A path for developers to go beyond smart contracts into **core Solana**.
* A foundation for **trustless interoperability** — by exporting transaction & storage finality proofs, Dummy Client bridges Solana into the broader multi-chain world.

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more information.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

*Ready to dive into Solana's protocol internals? Let's build the future of blockchain understanding together.*