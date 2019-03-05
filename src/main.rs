//Patrick Neilson 2019
//Project built following this video series by GeekLaunch: https://www.youtube.com/watch?v=vJdT05zl6jk
//
//A blockchain side project started in Feburary 2019
//
use patchainlib::*;
extern crate time;
use time::precise_time_ns;
fn now() -> u128 { precise_time_ns() as u128 }

fn main() {
    let difficulty = 1;
    let ledgersize = 10;
    
    let mut firstblock = Block::new(0, now(), vec![0; 32], 0, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                Output {
                    to_addr: "Alice".to_owned(),
                    value: 50
                },
                Output {
                    to_addr: "Bob".to_owned(),
                    value: 7
                }

            ],
        },
    ],
    difficulty);
    
    firstblock.mine();
    println!("First block mined: {:?}", &firstblock);
    let mut last_hash = firstblock.block_hash.clone();

    let mut blockchain = Chain::new();
    blockchain.update_with_block(firstblock).expect("Blockchain error");

    //Test a second block
    let mut block = Block::new(1, now(), last_hash, 0, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                Output {
                    to_addr: "Alice".to_owned(),
                    value: 50
                },
                Output {
                    to_addr: "Bob".to_owned(),
                    value: 7
                }

            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                Output {
                    to_addr: "Alice".to_owned(),
                    value: 36
                },
                Output {
                    to_addr: "Bob".to_owned(),
                    value: 12
                }

            ],
        }
    ],
    difficulty);
    
    block.mine();
    println!("Second block mined: {:?}", &block);
    let mut last_hash = block.block_hash.clone();
    blockchain.update_with_block(block).expect("Blockchain error");
}
