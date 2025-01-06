use block::Block;

mod block;

fn main() {
    println!("Hello, world!");
    let initial_block = Block::new(0, vec![0;32], "Genesis Block".to_owned());
    println!("{:?}", initial_block);
}


