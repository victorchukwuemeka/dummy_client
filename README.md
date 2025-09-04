# 🦁 Dummy Clients

**Dummy Clients** is a **full-feature educational Solana client**, built in Rust.
It’s not a production validator — instead, it’s a **learning-first implementation** of Solana’s networking, ledger, PoH, consensus, runtime, and RPC layers.

Our mission: give developers the ability to **understand, build, and hack the Solana protocol itself** without needing datacenter-grade hardware.

---

## 🚀 Features

* **Gossip Networking** → discover validators, slots, and cluster state
* **Ledger Store & Replay** → download and verify blocks locally
* **Proof of History Verifier** → replay Solana’s global clock
* **Consensus Simulator (Tower BFT)** → explore forks, voting, lockouts
* **Execution Runtime (Sealevel Lite)** → run transactions and programs
* **Custom RPC** → serve data directly from your local replayed ledger

All features are **real-world compatible** — same wire protocols, ledger format, and runtime behaviors as Solana Labs’ validator — but simplified and modular for clarity.

---

## 🧭 Why Build This?

Most Solana developers focus on dApps and smart contracts. Very few touch the **protocol internals**. Dummy Clients closes that gap.

By building and using it, you’ll:

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

---

