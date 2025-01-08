mod block;
mod blockchain;
mod transactions;

use blockchain::BlockChain;

fn main() {
    println!("Hello, world!");
    let mut joels_chain = BlockChain::new();
    joels_chain.add_new_block("Hello Joels new block".to_string());
}


