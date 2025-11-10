use solana_sdk::pubkey::Pubkey;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo{
    pub pubkey: Pubkey,
    pub gossip: SocketAddr,
    pub wallclock: u64
}


impl ContactInfo {
    pub fn new(pubkey: Pubkey, gossip:SocketAddr)->Self{
        Self { 
            pubkey, 
            gossip, 
            wallclock: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn to_bytes(&self)->Result<Vec<u8>, bincode::Error>{
        bincode::serialize(self)
    }

    pub fn from_bytes(bytes: &[u8])->Result<Self, bincode::Error>{
        bincode::deserialize(bytes)
    }
}
