use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::peer::PeerManager;

pub async fn send_heartbeat(peer_manager: &PeerManager) {
    let peers = peer_manager.get_peers().await;

    for peer in peers.iter() {
        if let Ok(mut stream) = tokio::net::TcpStream::connect(peer).await {
            if let Err(_) = stream.write_all(b"PING").await {
                peer_manager.remove_peer(peer).await;
            } else {
                let mut buffer = [0; 4];
                stream.read_exact(&mut buffer).await.expect("Failed to read response");
                if &buffer != b"PONG" {
                    peer_manager.remove_peer(peer).await;
                }
            }
        }
    }
}
