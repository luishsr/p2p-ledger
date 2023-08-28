use crate::models::Blockchain;
use crate::peer::PeerManager;
use tokio::io::{AsyncReadExt};

// Fetch and sync blockchain data with a given peer.
pub async fn sync_with_peer(peer_manager: &PeerManager, blockchain: &mut Blockchain) {
    let peers = peer_manager.get_peers().await;

    for peer in peers {
        if let Ok(mut stream) = tokio::net::TcpStream::connect(peer.clone()).await {
            let mut data = String::new();
            stream.read_to_string(&mut data).await.expect("Failed to read data");
            if let Ok(new_chain) = serde_json::from_str::<Blockchain>(&data) {
                // TODO: Implement consensus to decide which chain is valid.
                // For now, just replace the chain if the new one is longer.
                if new_chain.chain.len() > blockchain.chain.len() {
                    *blockchain = new_chain;
                }
            } else {
                peer_manager.remove_peer(&peer).await;
            }
        }
    }
}
