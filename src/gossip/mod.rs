pub mod message;
pub mod node;
pub mod transport;
pub mod solana;
pub mod solana_gossip;

use std::time::Duration;
use std::thread;
use std::net::SocketAddr;
//use anyhow::Ok;
use anyhow::Result;
use message::{Message, MessageType};
use transport::Transport;
use node::{Node, NodeStatus, Peer};

//pub use crate::gossip::GossipService;

pub struct GossipService{
    node :Node,
    transport : Transport,
    gossip_interval : Duration,
    fanout : usize 
}

impl GossipService {

    pub fn new(id: String, bind_addr:&str, fanout:usize)->Result<Self>{
        let transport = Transport::new(bind_addr)?;
        let address = transport.local_addr()?;
        let node = Node::new(id,address);

        Ok(Self {
            node,
            transport,
            gossip_interval:Duration::from_secs(1),
            fanout,
        })
    }

    pub fn current_timestamp()->u64{
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn add_seed(&mut self, peer_id:String, peer_addr: SocketAddr){
        let peer = Peer{
            id: peer_id,
            address: peer_addr,
            last_seen: Self::current_timestamp(),
            status: NodeStatus::Alive
        };
        self.node.add_peer(peer);
    }

    pub fn set_data(&mut self, key: String, value: Vec<u8>){
        let version = Self::current_timestamp();
        self.node.update_state(key, value, version);
    }

    fn gossip_round(&mut self){
        let peers = self.node.get_random_peers(self.fanout);

        for peer in peers{
            let mut message = Message::new(self.node.id.clone(), MessageType::Push);
            message.data = self.node.state.clone();
            
            if let Err(e) = self.transport.send(&message, &peer.address.to_string()) {
                eprintln!("Failed to send to {}: {}", peer.id, e);
            } else {
                println!("Sent gossip to {}", peer.id);
            }
        }
    }

    fn receive_messages(&mut self){
        loop {
            match self.transport.receive(){
                Ok((message, sender)) => {
                    println!("Received gossip from {}", message.sender_id);
                    self.node.merge_state(message.data);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No more messages, break
                    break;
                }
                Err(e) => {
                    eprintln!("Receive error: {}", e);
                    break;
                }
            }
        }
    }

    pub fn get_state(&self) -> &std::collections::HashMap<String, message::DataEntry> {
        &self.node.state
    }


    pub fn start(&mut self) {
        println!("Gossip service started: {}", self.node.id);
        
        loop {
            
            self.gossip_round();
            
            
            self.receive_messages();
            
            
            thread::sleep(self.gossip_interval);
        }
    }

}

