use std::collections::HashMap;
use std::net::SocketAddr;
use crate::gossip::message::DataEntry;

#[derive(Debug, Clone)]
pub enum NodeStatus{
    Alive,
    Suspect,
    Dead
}

#[derive(Debug, Clone)]
pub struct Peer{
    pub id: String,
    pub address: SocketAddr,
    pub last_seen: u64,
    pub status : NodeStatus
}

pub struct Node{
    pub id: String,
    pub address: SocketAddr,
    pub peers : HashMap<String, Peer>,
    pub state: HashMap<String, DataEntry>
}


impl Node {
    pub fn new(id:String, address: SocketAddr)->Self{
        Self { id, address, peers: HashMap::new(), state: HashMap::new() }
    }

    pub fn add_peer(&mut self, peer:Peer){
        self.peers.insert(peer.id.clone(), peer);
    }

    pub fn remove_peer(&mut self, peer_id: &str){
        self.peers.remove(peer_id);
    }

    pub fn update_state(&mut self, key:String, value:Vec<u8>, version:u64){
        self.state.entry(key).and_modify(|e|{
            if version > e.version {
                e.value = value.clone();
                e.version = version;
            }
        }).insert_entry(DataEntry { value, version });
    }

    pub fn merge_state(&mut self, incoming_state: HashMap<String, DataEntry>){
        for (key, incoming_entry) in incoming_state {
            self.state.entry(key).and_modify(|e|{
                if incoming_entry.version > e.version {
                    *e  = incoming_entry.clone();
                }
            }).insert_entry(incoming_entry);
        }
    }

    pub fn get_random_peers(&self, count:usize)->Vec<Peer>{
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        let alive_peers:Vec<Peer> = self.peers.values().filter(|p|
            matches!(p.status, NodeStatus::Alive)
        ).cloned().collect();

        let mut selected = alive_peers;
        selected.shuffle(&mut rng);
        selected.into_iter().take(count).collect()
    }


}