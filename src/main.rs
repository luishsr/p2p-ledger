mod models;
mod network;
mod peer;
mod gossip;
mod heartbeat;

use models::{Transaction, Blockchain};
use peer::Peer;
use crate::network::{PEER_ADDRESS, start_server, announce_to_peers};
use crate::peer::PeerManager;

#[tokio::main]
async fn main() {
    // Create a new blockchain instance
    let blockchain = Blockchain::new();

    // Define own peer address and create a new Peer instance
    let peer_address = String::from("127.0.0.1:8081");
    let peer = Peer::new(peer_address.clone(), blockchain);

    // Start listening for incoming connections
    tokio::spawn(async move {
        peer.listen().await;
    });

    // Add a dummy transaction and broadcast it to the network
    //let transaction = Transaction {
    //    sender: "Alice".to_string(),
    //    recipient: "Bob".to_string(),
    //    amount: 50,
    //};

    // Connect to a known peer, e.g., our main network address
    //peer.connect(PEER_ADDRESS).await;
    //peer.clone().send_transaction(PEER_ADDRESS, &transaction).await;

    // Create the peer manager
    let peer_manager = PeerManager::new();

    // Start P2P server
    start_server(&peer_manager, blockchain).await;

    loop {
        // Announce to peers every loop iteration
        network::announce_to_peers(&peer_manager).await;
    }



}
