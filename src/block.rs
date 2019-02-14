//The block datastructure of the blockchain

/*
Blocks contain this information:

Index - this block's location within the list of blocks
Payload - any relevant information or events that have occurred for/in the block
Timestamp - gives our blockchain a sense of time
Nonce - a special number used for mining (PoW verification)
Previous block hash - cryptographic fingerprint of previous block
Hash - cryptographic fingerprint of all of the above data concatenated together
*/

pub struct Block {}

impl Block {}