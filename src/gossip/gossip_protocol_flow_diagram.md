┌─────────────────────────────────────────────────────────────────┐
│                        GOSSIP PROTOCOL                           │
└─────────────────────────────────────────────────────────────────┘

1. BOOTSTRAP PHASE
==================

    New Node                    Seed Node                  Network
       │                           │                          │
       │──── JOIN Request ────────>│                          │
       │     (my_id, my_addr)      │                          │
       │                           │                          │
       │<─── Peer List ───────────│                          │
       │   [node1, node2, node3]   │                          │
       │                           │                          │
       └──────────────────────────────────────────────────────┘


2. GOSSIP ROUND (Every 1 second)
=================================

    Node A                    Node B                    Node C
      │                         │                         │
      │  Select Random Peers    │                         │
      │  (B, C from peer list)  │                         │
      │                         │                         │
      ├──── PUSH ──────────────>│                         │
      │   {                     │                         │
      │     sender: A           │   Receive & Merge       │
      │     data: {             │   Update local state    │
      │       key1: (val, v:5)  │                         │
      │       key2: (val, v:3)  │                         │
      │     }                   │                         │
      │   }                     │                         │
      │                         │                         │
      ├──── PUSH ──────────────────────────────────────>│
      │   (same data)                                    │
      │                                  Receive & Merge │
      │                                                   │


3. PUSH-PULL (Bidirectional)
=============================

    Node A                                Node B
      │                                     │
      │──── PUSH-PULL ────────────────────>│
      │   My State: {key1:v5, key2:v3}     │
      │                                     │
      │                Compare States       │
      │                key1: v5 vs v4 → A wins
      │                key3: none vs v7 → B wins
      │                                     │
      │<─── PUSH-PULL ─────────────────────│
      │   My State: {key3:v7, key4:v2}     │
      │                                     │
      Both nodes now have:                  │
      {key1:v5, key2:v3, key3:v7, key4:v2}  │
      │                                     │


4. STATE RECONCILIATION
=======================

    Before Gossip:
    
    Node A: {user1: "alice", version: 5}
    Node B: {user1: "bob", version: 7}
    Node C: {user1: "charlie", version: 3}
    
    After Round 1:
    
    A gossips to B → B keeps version 7 (higher)
    B gossips to C → C updates to version 7
    
    After Round 2:
    
    C gossips to A → A updates to version 7
    
    Final State (all nodes):
    {user1: "bob", version: 7}


5. FAILURE DETECTION
====================

    Node A                    Node B (crashed)          Node C
      │                           │                       │
      │                           X                       │
      │──── Heartbeat ───────────>X                       │
      │     (no response)                                 │
      │                                                   │
      │  Wait timeout (5s)                                │
      │  Mark B as SUSPECT                                │
      │                                                   │
      │──── Gossip to C ──────────────────────────────>│
      │   "Node B is SUSPECT"                            │
      │                                                   │
      │                           X                       │
      │  Wait another round                               │
      │  No heartbeat from B                              │
      │  Mark B as DEAD                                   │
      │  Remove from peer list                            │
      │                                                   │


6. COMPLETE GOSSIP CYCLE
=========================

    ┌─────────────────────────────────────────┐
    │         Node Event Loop                  │
    └─────────────────────────────────────────┘
              │
              ▼
    ┌─────────────────────┐
    │   Sleep 1 second     │
    └─────────────────────┘
              │
              ▼
    ┌─────────────────────┐
    │ Select k=3 random   │◄──── Fanout parameter
    │ peers from list     │
    └─────────────────────┘
              │
              ▼
    ┌─────────────────────┐
    │  For each peer:     │
    │  1. Send my state   │
    │  2. Receive state   │
    │  3. Merge states    │
    └─────────────────────┘
              │
              ▼
    ┌─────────────────────┐
    │ Update heartbeats   │
    │ Check for failures  │
    └─────────────────────┘
              │
              ▼
    ┌─────────────────────┐
    │ Clean dead peers    │
    └─────────────────────┘
              │
              └──────► Loop back


7. DATA PROPAGATION (EPIDEMIC)
===============================

    Round 0: Node A has new data
    
    [A*] [B] [C] [D] [E] [F]
    
    Round 1: A gossips to B and C (fanout=2)
    
    [A*] [B*] [C*] [D] [E] [F]
    
    Round 2: A,B,C each gossip to 2 random peers
    
    [A*] [B*] [C*] [D*] [E*] [F]
    
    Round 3: All nodes have the data
    
    [A*] [B*] [C*] [D*] [E*] [F*]
    
    Log(N) rounds to reach all nodes!


8. NETWORK PACKET FLOW
=======================

    Application Layer
         │
         │ Message{sender, timestamp, data}
         ▼
    Serialization (bincode)
         │
         │ [bytes: 0x48, 0x65, 0x6c, ...]
         ▼
    UDP Socket
         │
         │ Datagram packet
         ▼
    Network (Internet)
         │
         │ IP: 192.168.1.100 → 192.168.1.101
         ▼
    Receiving Node UDP Socket
         │
         │ [bytes received]
         ▼
    Deserialization
         │
         │ Message struct
         ▼
    State Merge Logic
         │
         ▼
    Updated Local State
```

## KEY COMPONENTS RELATIONSHIP
```
┌──────────────────────────────────────────────────────────┐
│                    GossipService                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │  Node (identity, state, peer_list)                 │  │
│  └────────────────────────────────────────────────────┘  │
│                         │                                 │
│                         │ uses                            │
│                         ▼                                 │
│  ┌────────────────────────────────────────────────────┐  │
│  │  Message (format, serialization)                   │  │
│  └────────────────────────────────────────────────────┘  │
│                         │                                 │
│                         │ sent via                        │
│                         ▼                                 │
│  ┌────────────────────────────────────────────────────┐  │
│  │  Transport (UDP socket, send/receive)              │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘