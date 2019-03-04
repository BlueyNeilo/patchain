//Patrick Neilson 2019
//Project built following this video series by GeekLaunch: https://www.youtube.com/watch?v=vJdT05zl6jk
//
//A blockchain side project started in Feburary 2019
//
use patchainlib::*;
extern crate time;
use time::precise_time_ns as now;

fn main() {
    let difficulty = 1;
    let ledgersize = 10;
    
    // let mut firstblock = Block::new(0, now() as u128, vec![0; 32], 0, "First block!".to_owned(), difficulty);
    // firstblock.mine();
    // println!("First block mined: {:?}", &firstblock);
    // let mut last_hash = firstblock.block_hash.clone();

    // let mut blockchain = Chain {
    //     blocks: vec![firstblock]
    // };

    // for i in 1..=ledgersize {
    //     let mut block = Block::new(i, now() as u128, last_hash, 0, "Another block..".to_owned(), difficulty);
    //     block.mine();
    //     println!("Block mined: {:?}", &block);
    //     last_hash = block.block_hash.clone();
    //     blockchain.blocks.push(block);
    // };
    // //blockchain.blocks[1].payload = "Test".to_owned();
    // blockchain.verify();

}
