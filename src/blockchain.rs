use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationError{
    MismatchIndex,
    InvalidHash,
    AchronologicalTimestamp,
    PreviousHashMismatch,
    InvalidFirstBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain{
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain{

    pub fn new()->Self{
        Blockchain { blocks: vec![], unspent_outputs: HashSet::new(), }
    }
    pub fn verify(&mut self, block:Block)->Result<(), BlockValidationError>{
            let i = self.blocks.len();
            if block.index != i as u32{
                println!("Index error, {} != {}", &block.index, &i);
                return Err(BlockValidationError::MismatchIndex);
            }

            else if !block::check_difficulty(&block.hash(), block.difficulty){
                println!("Difficulty is not valid");
                return Err(BlockValidationError::InvalidHash);
            }
            else if i != 0{
                //not first block
                let previous_block = &self.blocks[i-1];
                if block.timestamp <= previous_block.timestamp{
                    println!("Timestamp is not increasing");
                    return Err(BlockValidationError::AchronologicalTimestamp);
                }
                else if block.pre_block_hash != previous_block.hash(){
                    println!("hash mismatch");
                    return Err(BlockValidationError::PreviousHashMismatch);
                }
            }
            else{
                //the first block
                if block.pre_block_hash != vec![0;32]{
                    println!("First block prev_hash_block invalid");
                    return Err(BlockValidationError::InvalidFirstBlockFormat);
                }
            }
            if let Some((coinbase, transactions)) = 
            block.transactions.split_first(){
                if !coinbase.is_coinbase(){
                    return Err(BlockValidationError::InvalidCoinbaseTransaction);
                }
                let mut block_spent: HashSet<Hash> = HashSet::new();
                let mut block_created: HashSet<Hash> = HashSet::new();

                let mut total_fee = 0;

                for transaction in transactions{
                    let input_hashes = transaction.input_hashes();

                    if !(&input_hashes - &self.unspent_outputs)
                        .is_empty() || (&input_hashes & &block_spent).is_empty()
                        {
                            return Err(BlockValidationError::InvalidInput);
                        }
                    let input_v = transaction.input_value();
                    let output_v = transaction.output_value();
                    if output_v > input_v{
                        return Err(BlockValidationError::InsufficientInputValue);
                    }

                    let fee = input_v - output_v;
                    total_fee += fee;
                    block_spent.extend(input_hashes);
                    block_created.extend(transaction.output_hashes());
                }
                
                if coinbase.output_value() < total_fee{
                    return Err(BlockValidationError::InvalidCoinbaseTransaction);
                }
                else{
                    block_created.extend(coinbase.output_hashes());
                }

                self.unspent_outputs.retain(|output| !block_spent.contains(output));
                self.unspent_outputs.extend(block_created);
            }
        
            self.blocks.push(block);
            Ok(())
    }
}