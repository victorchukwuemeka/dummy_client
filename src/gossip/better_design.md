### Solana Gossip Protocol Bridge Design – **Updated for Agave v2.0 (Nov 2025)**

**Key Updates from Latest Specs** (based on Agave v2.0 mainnet launch, Oct 2024; no major gossip changes in v2.2, Jul 2025):  
- **Crate Renames**: `solana-*` → `agave-*` (e.g., `agave-sdk`). Update scripts/automation.  
- **Gossip Unchanged**: Core protocol (6 message types, CRDS, push/pull/prune) stable since v1.18. Spy mode via `agave-gossip` CLI.  
- **Performance**: Agave v2.0 boosts validator efficiency (95%+ stake adoption); gossip now stake-weighted for better resilience.  
- **Multi-Client**: Compatible with Firedancer; use `extensions` in `ContactInfo` for future clients.  
- **No Breaking Changes**: Backward-compatible; deprecated RPCs (e.g., `getVoteAccounts`) don't affect gossip.

---

```
┌─────────────────────────────────────────────────────────────────┐
│ CURRENT STATE (Generic Gossip)                                  │
└─────────────────────────────────────────────────────────────────┘
Your Generic Gossip:
  Message { sender_id, timestamp, data: HashMap<String, bytes> }
                                         ^^^^^^^^^^^^^^^^^^^^^^
                                         Any random data
┌─────────────────────────────────────────────────────────────────┐
│ TARGET STATE (Solana-Aware Gossip – Agave v2.0)                 │
└─────────────────────────────────────────────────────────────────┘
Solana Gossip:
  Message { sender_id, timestamp, data: CrdsValue }  // Signed CRDS payload
                                        ^^^^^^^^^^^^
                                        Agave-validated blockchain data
```

---

```
═══════════════════════════════════════════════════════════════════
                        THE BRIDGE (What We Need – v2.0)
═══════════════════════════════════════════════════════════════════
1. UNDERSTAND AGAVE DATA STRUCTURES (CRDS, ContactInfo v2)
   ↓
2. DEFINE SOLANA MESSAGE TYPES (6 Protocol variants, exact IDs)
   ↓
3. SERIALIZE/DESERIALIZE SOLANA DATA (bincode LE, 1232B max)
   ↓
4. INTEGRATE WITH YOUR GOSSIP PROTOCOL (Feed bytes to generic engine)
   ↓
5. CONNECT TO REAL SOLANA NETWORK (IP Echo + entrypoint join)
```

---

```
              STEP 1: SOLANA DATA STRUCTURES WE NEED (v2.0)
═══════════════════════════════════════════════════════════════════
```

┌────────────────────────────────────────────────────────────────┐  
│ **ContactInfo** – How to reach a validator (v2.0 w/ extensions) │  
├────────────────────────────────────────────────────────────────┤  
│ - pubkey: Pubkey (validator identity)                          │  
│ - gossip: SocketAddr (gossip port, UDP 8000-10000 range)       │  
│ - tvu: SocketAddr (transaction verification)                   │  
│ - tpu: SocketAddr (transaction processing)                     │  
│ - rpc: SocketAddr (RPC endpoint)                               │  
│ - version: Version (Agave v2.0+, client ID: 3=Agave)           │  
│ - shred_version: u16 (cluster compat check)                    │  
│ - wallclock: u64 (timestamp, < MAX_WALLCLOCK)                  │  
│ - outset: u64 (instance start time)                            │  
│ - addrs: Vec<IpAddr> (IPv4/IPv6)                               │  
│ - sockets: Vec<SocketEntry> (keyed ports: gossip=0, tpu=5)     │  
│ - extensions: Vec<Extension> (future multi-client, e.g., Firedancer) │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **Vote** – Validator voting on blocks                          │  
├────────────────────────────────────────────────────────────────┤  
│ - index: u8 (tower slot % 32)                                  │  
│ - from: Pubkey (who voted)                                     │  
│ - transaction: Transaction (signed vote tx)                    │  
│ - wallclock: u64 (when)                                        │  
│ - slot: Option<Slot> (voted slot)                              │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **EpochSlots** – Slot completion info                          │  
├────────────────────────────────────────────────────────────────┤  
│ - index: u8 (epoch slot % 32)                                  │  
│ - from: Pubkey (reporting validator)                           │  
│ - slots: Vec<CompressedSlots> (Flate2/Uncompressed)            │  
│ - wallclock: u64 (timestamp)                                   │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **LegacyContactInfo** – Older format (deprecated, compat only) │  
├────────────────────────────────────────────────────────────────┤  
│ Similar to ContactInfo but flat SocketAddr fields (no addrs/sockets) │  
└────────────────────────────────────────────────────────────────┘  

---

```
         STEP 2: SOLANA GOSSIP MESSAGE ENVELOPE (CrdsValue – v2.0)
═══════════════════════════════════════════════════════════════════
```

Solana wraps all gossip data in **CrdsValue** (Cluster Replicated Data Store – GossipTable in Agave).  

┌────────────────────────────────────────────────────────────────┐  
│ **CrdsValue**                                                  │  
├────────────────────────────────────────────────────────────────┤  
│ signature: Signature ([u8; 64], Ed25519 over data)             │  
│ data: CrdsData (enum w/ 4B discriminant)                       │  
│ ↓                                                              │  
│ **enum CrdsData** (exact IDs, bincode LE):                     │  
│   0: LegacyContactInfo(LegacyContactInfo)                      │  
│   1: Vote(u8, Vote)                                            │  
│   2: LowestSlot(u8, LowestSlot)                                │  
│   3: LegacySnapshotHashes(AccountsHashes)                      │  
│   4: AccountsHashes(AccountsHashes)                            │  
│   5: EpochSlots(u8, EpochSlots)                                │  
│   6: LegacyVersion(LegacyVersion)                              │  
│   7: Version(Version)                                          │  
│   8: NodeInstance(NodeInstance)                                │  
│   9: DuplicateShred(u16, DuplicateShred)                       │  
│  10: SnapshotHashes(SnapshotHashes)                            │  
│  11: ContactInfo(ContactInfo)                                  │  
│  12: RestartLastVotedForkSlots(RestartLastVotedForkSlots)     │  
│  13: RestartHeaviestFork(RestartHeaviestFork)                  │  
│ // ... (deprecated variants marked)                            │  
└────────────────────────────────────────────────────────────────┘  

---

```
              STEP 3: SOLANA GOSSIP PROTOCOL FLOW (v2.0)
═══════════════════════════════════════════════════════════════════
```

┌──────────────┐ ┌──────────────┐  
│ Validator A  │ │ Validator B  │  
└──────────────┘ └──────────────┘  
       │ │  
       │ 1. Create CrdsValue (e.g., ContactInfo) │  
       │ 2. Sign w/ private key (Ed25519) │  
       │ 3. Push to stake-weighted peers (fanout=9) │  
       ├─────────── PushMessage(Pubkey, Vec<CrdsValue>) ──────>│  
       │ │  
       │ 4. Verify sig + sanitize (wallclock < MAX) │  
       │ 5. Upsert to CRDS (GossipTable) │  
       │ 6. Propagate (if stake > 0, update receive cache) │  
       │ │  
       │<────────── PullRequest(CrdsFilter, ContactInfo) ──────┤  
       │ │  
       │ 7. Lookup shards (12 bits), filter Bloom, respond │  
       ├─────────── PullResponse(Pubkey, Vec<CrdsValue>) ──────>│  
       │ │  

*(Prune if upserts >=20; Ping/Pong for liveness; Rotate active set every 7.5s.)*

---

```
                STEP 4: OUR IMPLEMENTATION STRATEGY (v2.0)
═══════════════════════════════════════════════════════════════════
```

Current Structure: New Solana Structure:  
`src/gossip/` → `src/gossip/` (keep generic: message.rs, node.rs, transport.rs)  
                                    └── `solana/` (Agave v2.0 driver)  
                                        ├── mod.rs  
                                        ├── crds.rs (CrdsValue + exact CrdsData enum)  
                                        ├── contact_info.rs (v2.0 w/ extensions)  
                                        ├── vote.rs (Vote + Transaction)  
                                        ├── epoch_slots.rs (CompressedSlots)  
                                        ├── protocol.rs (6 message types, join flow)  
                                        ├── push_active_set.rs (25 buckets, stake-weighted)  
                                        ├── prune.rs (receive cache, 0.15 threshold)  
                                        ├── pull.rs (CrdsFilter, mask_bits=log2(items/2000))  
                                        ├── ip_echo.rs (TCP req/resp for shred_version)  
                                        └── cluster.rs (GossipTable view: validators, stakes)  

---

```
              STEP 5: DATA FLOW (Generic → Solana v2.0)
═══════════════════════════════════════════════════════════════════
```

**BEFORE (Generic):**  
┌─────────────────────────────────────────────────────────────────┐  
│ Application │  
│ ↓ set_data("key", bytes) │  
│ GossipService │  
│ ↓ store in HashMap<String, bytes> │  
│ Transport │  
│ ↓ send bytes over UDP │  
│ Network │  
└─────────────────────────────────────────────────────────────────┘  

**AFTER (Solana v2.0):**  
┌─────────────────────────────────────────────────────────────────┐  
│ Agave Application │  
│ ↓ create ContactInfo { pubkey, sockets, extensions, ... } │  
│ SolanaGossipService │  
│ ↓ wrap in CrdsValue (ID=11) │  
│ ↓ sign (Ed25519 over data) │  
│ ↓ bincode serialize (LE, ≤1232B) │  
│ GossipService (reuse!) │  
│ ↓ store/upsert CRDS + propagate │  
│ Transport (reuse!) │  
│ ↓ send over UDP (gossip port) │  
│ Network → Real Solana Validators (95%+ on Agave v2.0) │  
└─────────────────────────────────────────────────────────────────┘  

---

```
              STEP 6: WHAT EACH FILE WILL DO (v2.0)
═══════════════════════════════════════════════════════════════════
```

┌────────────────────────────────────────────────────────────────┐  
│ **solana/crds.rs** – Cluster Replicated Data Store (GossipTable) │  
├────────────────────────────────────────────────────────────────┤  
│ struct CrdsValue { signature: Signature, data: CrdsData } │  
│ enum CrdsData { ... } (exact 14 variants w/ IDs) │  
│ Methods: │  
│ - new(data: CrdsData) → CrdsValue │  
│ - sign(keypair: &Keypair) → signed │  
│ - verify() → bool (sig over data) │  
│ - serialize() → Vec<u8> (bincode LE) │  
│ - deserialize(bytes: &[u8]) → Result<CrdsValue> │  
│ - upsert_to_crds(table: &mut IndexMap) │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **solana/contact_info.rs** – Validator network info (v2.0)      │  
├────────────────────────────────────────────────────────────────┤  
│ struct ContactInfo { ... extensions: Vec<Extension> } │  
│ Methods: │  
│ - new(pubkey: Pubkey, sockets: Vec<SocketEntry>) → ContactInfo │  
│ - is_valid() → bool (shred_version match, wallclock fresh) │  
│ - to_crds_value(keypair: &Keypair) → CrdsValue (ID=11) │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **solana/vote.rs** – Validator votes                            │  
├────────────────────────────────────────────────────────────────┤  
│ struct Vote { index: u8, from: Pubkey, transaction: Transaction, ... } │  
│ Methods: │  
│ - new(from: Pubkey, slots: Vec<Slot>) → Vote │  
│ - to_crds_value(keypair: &Keypair) → CrdsValue (ID=1) │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **solana/protocol.rs** – Solana-specific gossip logic (v2.0)    │  
├────────────────────────────────────────────────────────────────┤  
│ struct SolanaGossipService { │  
│   gossip: GossipService, // reuse generic │  
│   crds_table: IndexMap<CrdsValueLabel, VersionedCrdsValue>, │  
│   keypair: Keypair, │  
│   active_set: PushActiveSet, │  
│ } │  
│ Methods: │  
│ - publish_contact_info() – Push to fanout=9 peers │  
│ - publish_vote(vote: Vote) – Gossip signed vote │  
│ - handle_crds_value(bytes: &[u8]) – Verify + upsert │  
│ - connect_to_cluster(entrypoint: SocketAddr) – IP Echo + join │  
└────────────────────────────────────────────────────────────────┘  

┌────────────────────────────────────────────────────────────────┐  
│ **solana/cluster.rs** – Cluster state management (GossipTable view) │  
├────────────────────────────────────────────────────────────────┤  
│ struct ClusterInfo { │  
│   validators: HashMap<Pubkey, ContactInfo>, │  
│   stakes: HashMap<Pubkey, Stake>, │  
│   // Derived from CRDS │  
│ } │  
│ Methods: │  
│ - get_validators() → Vec<Pubkey> │  
│ - get_stakes() → &HashMap<Pubkey, Stake> │  
│ - get_tpu_addresses() → Vec<SocketAddr> │  
└────────────────────────────────────────────────────────────────┘  

---

```
              STEP 7: CONNECTING TO REAL SOLANA NETWORK (v2.0)
═══════════════════════════════════════════════════════════════════
```

1. **Get Solana Entrypoint Addresses** (Agave v2.0 compatible)  
   ↓  
   Mainnet: `entrypoint.mainnet-beta.solana.com:8001`  
   Testnet: `entrypoint.testnet.solana.com:8001`  
   Devnet: `entrypoint.devnet.solana.com:8001`  

2. **Connect to Entrypoint**  
   ↓  
   TCP IP Echo → get pub IP + shred_version  

3. **Receive Node List**  
   ↓  
   Push Version/NodeInstance → PullRequest → get ContactInfo  

4. **Start Gossiping**  
   ↓  
   Bind UDP gossip → Push/Pull loop (every 7.5s rotate)  

5. **Monitor Cluster**  
   ↓  
   Track CRDS: votes, slots, SnapshotHashes (download if spy→gossip)  

*(Spy mode: random port 8000-10000, shred_version=0; full mode post-snapshot.)*

---

```
                  STEP 8: IMPLEMENTATION PHASES (v2.0 Order)
═══════════════════════════════════════════════════════════════════
```

**PHASE 1: Data Structures (3 files)**  
├── Define Pubkey/Signature (agave-sdk)  
├── Create ContactInfo (v2.0 w/ extensions)  
├── Create CrdsValue wrapper (exact IDs)  
└── Test bincode ser/de (≤1232B)  

**PHASE 2: CRDS Store (1 file)**  
├── IndexMap<CrdsValueLabel, VersionedCrdsValue> (upsert w/ hash/shards)  
├── Handle updates (timestamp wins)  
├── Prune old (wallclock -30s)  
└── Test mock upserts  

**PHASE 3: Solana Protocol Layer (3 files)**  
├── Wrap generic w/ SolanaGossipService  
├── Sign/verify (Ed25519, prefix for Prune)  
├── Handle push/pull/prune (Bloom + shards)  
└── Test local multi-node (2 validators sync CRDS)  

**PHASE 4: Real Network Integration (2 files)**  
├── IP Echo TCP client (udp/tcp ports check)  
├── Join entrypoint (PullResponse → SnapshotHashes)  
├── Parse real packets (sanitize + verify)  
└── Monitor mainnet-beta (log validators/stakes)  

**PHASE 5: Advanced Features**  
├── Stake-weighted active set (25 buckets, log2(stake))  
├── Prune at 20 upserts (0.15 min_stake threshold)  
├── Turbine integration (shred prop)  
└── Multi-client sim (Firedancer compat via extensions)  

---

```
                      DEPENDENCIES NEEDED (Nov 2025)
═══════════════════════════════════════════════════════════════════
```

```toml
[dependencies]
agave-sdk = "2.2"     # Pubkey, Signature, Keypair (renamed from solana-sdk)
agave-gossip = "2.2"  # Spy CLI reference (renamed from solana-gossip)
ed25519-dalek = "2.0" # Signing
bincode = "1.3"       # Ser/de (LE, dynamic arrays)
serde = { version = "1.0", features = ["derive"] }
bloomfilter = "1.0"   # CrdsFilter impl
indexmap = "2.0"      # CRDS table
```

*(No pip/wget; use Cargo. Multi-client: agave + firedancer crates incoming 2026.)*

---

```
                     FINAL ARCHITECTURE (v2.0)
═══════════════════════════════════════════════════════════════════
```

```
┌─────────────────────┐
│   Your Application  │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ SolanaGossipService │ ← New Layer (Agave v2.0-specific)
│ (CRDS upsert, sign) │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│   GossipService     │ ← Reuse (generic: push/pull/merge)
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│     Transport       │ ← Reuse (UDP, 8001 port)
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ Solana Network      │
│ (Real validators,   │
│  95%+ on Agave v2.0)│
└─────────────────────┘
```

---

```
                          SUMMARY (v2.0)
═══════════════════════════════════════════════════════════════════
```

✅ **Keep your generic gossip** (it works! Feed it signed bytes)  
✅ **Add Solana layer on top** (CRDS + exact enums)  
✅ **Define Solana data structures** (ContactInfo v2.0, Vote, extensions)  
✅ **Wrap in CrdsValue w/ signatures** (Ed25519, bincode LE)  
✅ **Reuse your transport/protocol** (UDP push/pull)  
✅ **Connect to real Solana validators** (IP Echo + mainnet-beta:8001)  

Your generic gossip becomes the **ENGINE**.  
Solana layer becomes the **DRIVER** – spec-perfect for Agave v2.0.  

