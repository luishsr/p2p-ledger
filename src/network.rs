use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::models::Blockchain;
use crate::peer::PeerManager;
use crate::gossip::sync_with_peer;
use crate::heartbeat::send_heartbeat;

pub const PEER_ADDRESS: &str = "127.0.0.1:8085";

pub async fn announce_to_peers(peer_manager: &PeerManager) {
    let peers = peer_manager.get_peers().await;
    for peer in &peers {
        let peer_addr = format!("{}:8080", peer);  // Assuming all peers use port 8080
        if let Ok(mut stream) = tokio::net::TcpStream::connect(peer_addr).await {
            stream.write_all(b"ANNOUNCE").await.expect("Failed to announce");
        }
    }
}
pub async fn start_server(peer_manager: &PeerManager, mut blockchain: Blockchain) {
    let listener = TcpListener::bind(PEER_ADDRESS).await.expect("Unable to bind to address");
    println!("Listening on {}", PEER_ADDRESS);

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                let peer_address = addr.to_string();
                let mut buffer = [0; 4];
                socket.read_exact(&mut buffer).await.expect("Failed to read request");

                if &buffer == b"PING" {
                    socket.write_all(b"PONG").await.expect("Failed to write response");
                } else {
                    // Assuming other requests are for the blockchain
                    let chain_data = serde_json::to_string(&blockchain.chain).unwrap();
                    socket.write_all(chain_data.as_bytes()).await.expect("Failed to write data");

                    // Register the peer since it's requesting blockchain data
                    peer_manager.register_peer(peer_address).await;
                }
            }
            Err(e) => eprintln!("Failed to accept connection: {:?}", e),
        }

        // Sync with peers every loop iteration
        sync_with_peer(&peer_manager, &mut blockchain).await;

        // Send heartbeat to peers
        send_heartbeat(&peer_manager).await;
    }


}
