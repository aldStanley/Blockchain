use blockchainlib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;
    let mut first_block = Block::new(0, now(), vec![0;32]
                            , vec![
                                Transaction{
                                    inputs: vec![],
                                    outputs: vec![
                                        transaction::Output{
                                            to_address: "Alice".to_owned(),
                                            value: 50,
                                        },
                                        transaction::Output{
                                            to_address: "Bob".to_owned(),
                                            value:7,
                                        }
                                    ]
                                }
                            ], difficulty);

    first_block.hash = first_block.hash();
    println!("{:?}", &first_block);

    first_block.mine();
    println!("{:?}", &first_block);

    let mut last_hash = first_block.hash.clone();

    let mut blockchain = Blockchain::new();

    //test block 2

    let mut block = Block::new(1, now(), last_hash
                            , vec![
                                Transaction{
                                    inputs: vec![],
                                    outputs: vec![
                                        transaction::Output{
                                            to_address: "Stanley".to_owned(),
                                            value: 100,
                                        },
                                        transaction::Output{
                                            to_address: "Ben".to_owned(),
                                            value:9,
                                        }
                                    ]
                                }
                            ], difficulty);


    block.mine();

    last_hash = block.hash.clone();

    blockchain.verify(block).expect("Failed to add the new block!");
}
