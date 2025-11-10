use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Push,
    Pull,
    PushPull
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message{
    pub sender_id : String,
    pub timestamp: u64,
    pub message_type: MessageType,
    pub data : HashMap<String, DataEntry>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry{
    pub value: Vec<u8>,
    pub version : u64
}


impl Message {
    pub fn new(sender_id: String, message_type :MessageType)->Self{
        Self{
            sender_id,
            timestamp : std::time::SystemTime::now()
              .duration_since(std::time::UNIX_EPOCH)
              .unwrap()
              .as_secs(),
            message_type,
            data:HashMap::new()
        }
    }

    pub fn to_bytes(&self)->Result<Vec<u8>, bincode::Error>{
        bincode::serialize(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

}