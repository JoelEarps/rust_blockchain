use sha2::digest::block_buffer::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

struct PeerToPeerNetworkHandler {

}

impl PeerToPeerNetworkHandler {
    async fn broadcast_to_all_nodes(){
        todo!()
    }
}

pub struct PeerNode {
    buffer: Vec<i32>,
    listener: TcpListener
}

// Purpose of this is to play the roll of the node
/*
Requirements of a Node:
1. Join the network 
2. Listen for incoming traffic and changes to the blockchain
3. Synchronise periodically to keep up to date.
 */
// Does this need to be a vector - should it be an array, why is 1024?
// Are there other things that could be used
impl PeerNode {
    // Custom Error for TCP bind Error
    pub async fn new() -> Self {
        // Handle bind errors here
        Self { 
        buffer: vec![0; 1024],
        listener: TcpListener::bind("127.0.0.1:7878").await.expect("Custom Message for Peer Node Bind Error for handling")
        }
    }


   // TODO: Refactor to match network diagram for P2P network
   // How should this work in line with Requirements for Network and Node in general
    pub async fn listen_for_incoming_traffic(&mut self){
        loop{
        let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];
                    // In a loop, read data from the socket and write the data back.
                    loop {
                        let n = socket.read(&mut buf).await.unwrap();
                        if n == 0 { break; }
                        socket.write_all(&buf[0..n]).await.unwrap();
                    }
                });
            }
        }
    }

    pub fn synchronise_with_network(){
        todo!()
    }
}