mod block;
mod blockchain;
mod transactions;
mod peer_to_peer_network;
mod consensus_mechanisms;

use blockchain::BlockChain;
use peer_to_peer_network::PeerNode;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // Custom Error here for this
    let mut network_node = PeerNode::new().await;
    network_node.listen_for_incoming_traffic().await;
    let mut joels_chain = BlockChain::new();
    joels_chain.add_new_block("Hello Joels new block".to_string());
}


