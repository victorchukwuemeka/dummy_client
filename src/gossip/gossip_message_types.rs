/**
 * Solana nodes communicate with each other and share data using the gossip protocol. Messages are exchanged in a binary format and need to be deserialized. There are six types of messages:

    pull request
    pull response
    push message
    prune message
    ping
    pong
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Push,
    Pull,
    Prune,
    Ping,
    Pong,
    PushPull
}

