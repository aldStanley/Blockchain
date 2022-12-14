use super::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Output{
    pub to_address: Address,
    pub value: u64,
}

impl Hashable for Output{
    fn bytes(&self)->Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_address.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}
pub struct Transaction{
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction{
    pub fn input_value (&self)-> u64{
        self.inputs.iter().map(|inputs|inputs.value).sum()
    }

    pub fn output_value (&self)->u64{
        self.outputs.iter().map(|outputs|outputs.value).sum()
    }

    pub fn input_hashes(&self)->HashSet<Hash>{
        self.inputs.iter().map(|inputs| inputs.hash()).collect::<HashSet<Hash>>()
    }

    pub fn output_hashes(&self)->HashSet<Hash>{
        self.outputs.iter().map(|outputs| outputs.hash()).collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase(&self)->bool{
        self.inputs.len() == 0 
    }
}

impl Hashable for Transaction{
    fn bytes(&self)->Vec<u8> {
        let mut bytes = vec![];
        
        bytes.extend(self.inputs
                               .iter()
                               .flat_map(|inputs| inputs.bytes())
                               .collect::<Vec<u8>>());
                    
        bytes.extend(self.outputs
                         .iter()
                         . flat_map(|outputs| outputs.bytes())
                         . collect::<Vec<u8>>());
        bytes
    }
}