extern crate blockchain_lib;

use blockchain_lib::*;

pub fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut blockchain = Blockchain::new();

    let mut gen_block_output_transaction = Transaction::new();
    gen_block_output_transaction.add_output_value("Alice".to_owned(), 50);
    gen_block_output_transaction.add_output_value("Bob".to_owned(), 20);

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![gen_block_output_transaction],
        difficulty,
    );
    genesis_block.mine();

    let last_hash = genesis_block.hash.clone();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block.");

    let mut block_input_transaction = Transaction::new();
    block_input_transaction.add_output_value("Alice".to_owned(), 50);

    let mut block_output_transaction1 = Transaction::new();
    block_output_transaction1.add_input_value("Alice".to_owned(), 50);
    block_output_transaction1.add_output_value("Alice".to_owned(), 36);
    block_output_transaction1.add_output_value("Bob".to_owned(), 12);

    let mut block_output_transaction2 = Transaction::new();
    block_output_transaction2.add_input_value("Bob".to_owned(), 20);
    block_output_transaction2.add_output_value("Alice".to_owned(), 16);
    block_output_transaction2.add_output_value("Bob".to_owned(), 3);

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            block_input_transaction,
            block_output_transaction1,
            block_output_transaction2,
        ],
        difficulty,
    );
    block.mine();

    blockchain
        .update_with_block(block)
        .expect("Failed to add block.");
}
