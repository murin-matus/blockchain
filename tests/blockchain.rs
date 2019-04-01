#[cfg(test)]
mod tests {
    extern crate blockchain_lib;
    use self::blockchain_lib::*;

    #[test]
    fn genesis_block_mine() {
        let calculated_hash = "7c2667e34376079b59d4653c41a95c648d10b46840bdaedeb8a62543f4119247";
        let difficulty = 0xffffffffffffffffffffffffffffffff;
        let timestamp = 1554050560890;

        let mut blockchain = Blockchain::new();

        let mut gen_block_output_transaction = Transaction::new();
        gen_block_output_transaction.add_output_value("Alice".to_owned(), 50);
        gen_block_output_transaction.add_output_value("Bob".to_owned(), 20);

        let mut genesis_block = Block::new(
            0,
            timestamp,
            vec![0; 32],
            vec![Transaction {
                inputs: vec![],
                outputs: vec![
                    Output::new("Alice".to_owned(), 50),
                    Output::new("Bob".to_owned(), 20),
                ],
                timestamp: timestamp,
            }],
            difficulty,
        );
        genesis_block.mine();
        assert_eq!(hex::encode(&genesis_block.hash), calculated_hash);

        &blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block.");
        assert_eq!(blockchain.blocks.len(), 1);
    }
}
