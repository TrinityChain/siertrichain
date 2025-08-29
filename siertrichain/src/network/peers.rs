use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Peer {
    pub address: SocketAddr,
    pub reputation: i32,
    // Add more peer-specific data as needed
}

pub struct PeerManager {
    peers: HashMap<SocketAddr, Peer>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self { peers: HashMap::new() }
    }

    pub fn add_peer(&mut self, address: SocketAddr) {
        self.peers.entry(address).or_insert(Peer {
            address,
            reputation: 0,
        });
    }

    pub fn get_peer(&self, address: &SocketAddr) -> Option<&Peer> {
        self.peers.get(address)
    }

    pub fn remove_peer(&mut self, address: &SocketAddr) {
        self.peers.remove(address);
    }

    // Add more methods for reputation management and peer selection
}
