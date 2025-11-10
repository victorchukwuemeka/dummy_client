use crate::gossip::{GossipService, message::MessageType};
use crate::gossip::solana::ContactInfo;

use solana_sdk::signature::{Keypair, Signer};
use std::collections::HashMap;
use anyhow::{Result};
use std::result::Result::Ok;

pub struct SolanaGossipService{
    gossip: GossipService,
    keypair: Keypair,
}


impl SolanaGossipService {
    pub fn new(bind_addr:&str, fanout:usize)->Result<Self>{
        let keypair  = Keypair::new();
        let id = keypair.pubkey().to_string();
        let gossip = GossipService::new(id, bind_addr, fanout)?;
        Ok(Self { gossip, keypair })
    }

    pub fn publish_contact_info(&mut self, contact:ContactInfo)->Result<()>{
        let key  = contact.pubkey.to_string();
        let value = contact.to_bytes()?;

        self.gossip.set_data(key, value);
        Ok(())
    }

    pub fn get_cluster(&self)->HashMap<String,ContactInfo>{
        let mut cluster = HashMap::new();
        
        for(key, entry)in self.gossip.get_state(){
            match ContactInfo::from_bytes(&entry.value) {
                Ok(contact) => {
                    cluster.insert(key.clone(), contact);
                }

                Err(e)=>{
                     eprintln!("Invalid contact info for key {}: {}", key, e);
                }
            }
        }

        cluster 
    }

}