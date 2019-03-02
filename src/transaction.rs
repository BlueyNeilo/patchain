use super::*;
use std::collections::HashSet;

type Address = String;

pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&self.value.to_be_bytes());

        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn input_valuesum(&self) -> u64 {
        self.valuesum(&self.inputs)
    }

    pub fn output_valuesum(&self) -> u64 {
        self.valuesum(&self.outputs)
    }

    pub fn input_hashes(&self) -> HashSet<Vec<u8>> {
        self.hashes(&self.inputs)
    }

    pub fn output_hashes(&self) -> HashSet<Vec<u8>> {
        self.hashes(&self.outputs)
    }
    
    fn valuesum(&self, outs: &Vec<Output>) -> u64 {
        outs.iter().map(|o| o.value).sum()
    }
    
    fn hashes(&self, outs: &Vec<Output>) -> HashSet<Vec<u8>> {
        outs.iter()
            .map(|o| o.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>()
        );
        
        bytes
    }
}