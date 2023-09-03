use std::collections::HashSet;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::{Blockchain, Transaction};
use crate::gossip::sync_with_peer;
use crate::heartbeat::send_heartbeat;

pub struct Peer {
    pub address: String,
    blockchain: Arc<Mutex<Blockchain>>,
}

pub struct PeerManager {
    peers: Arc<Mutex<HashSet<String>>>,
}

impl PeerManager {
    pub fn new() -> Self {
        PeerManager {
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    // Register a new peer.
    pub async fn register_peer(&self, address: String) {
        let mut lock = self.peers.lock().await;
        lock.insert(address);
    }

    // Remove a peer.
    pub async fn remove_peer(&self, address: &String) {
        let mut lock = self.peers.lock().await;
        lock.remove(address);
    }

    // Get a list of all peers.
    pub async fn get_peers(&self) -> HashSet<String> {
        self.peers.lock().await.clone()
    }
}

impl Peer {
    pub fn new(address: String, blockchain: Blockchain) -> Self {
        Peer {
            address,
            blockchain: Arc::new(Mutex::new(blockchain)),
        }
    }

    // Connect to a peer by address
    pub async fn connect(&self, peer_addr: &str) {
        let mut stream = TcpStream::connect(peer_addr).await.expect("Unable to connect to peer");
        stream.write_all(b"CONNECTX").await.expect("Failed to send CONNECT message");
    }

    // Listen for incoming connections
    pub async fn listen(&self) {
        let listener = TcpListener::bind(&self.address).await.expect("Unable to bind to address");
        loop {
            let (mut socket, addr) = listener.accept().await.expect("Failed to accept connection");
            let cloned_blockchain = self.blockchain.clone();

            // Create the peer manager
            let peer_manager = PeerManager::new();

            tokio::spawn(async move {
                let peer_address = addr.to_string();
                let mut buffer = [0; 8];
                socket.read_exact(&mut buffer).await.expect("Failed to read data");

                match &buffer {
                    b"CONNECTX" => {
                        println!("New peer connected!");
                        // Register the peer since it's requesting blockchain data
                        peer_manager.register_peer(peer_address).await;
                    },
                    b"TRANSACT" => {
                        let mut data = String::new();
                        socket.read_to_string(&mut data).await.expect("Failed to read data");
                        if let Ok(transaction) = serde_json::from_str::<Transaction>(&data) {
                            let mut bc = cloned_blockchain.lock().await;
                            bc.add_block(vec![transaction]);
                        }
                    },
                    _ => {}
                }
            });

            // Sync with peers every loop iteration
            let mut this_blockchain = self.get_blockchain().await;
            sync_with_peer(&peer_manager, &mut this_blockchain ).await;

            // Send heartbeat to peers
            send_heartbeat(&peer_manager).await;

        }
    }

    // Send a transaction to a peer
    pub async fn send_transaction(&self, peer_addr: &str, transaction: &Transaction) {
        let mut stream = TcpStream::connect(peer_addr).await.expect("Unable to connect to peer");
        stream.write_all(b"TRANSACT").await.expect("Failed to send TRANSACT message");
        stream.write_all(serde_json::to_string(transaction).unwrap().as_bytes()).await.expect("Failed to send transaction data");
    }

    // Getters
    pub async fn get_blockchain(&self) -> Blockchain {
        let mut bc = self.blockchain.lock().await;
        bc.into_inner()
    }
}