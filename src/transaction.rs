use super::*;
use std::collections::HashSet;

type Address = String;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes (&self) -> Hash {
        vec![self.to_addr.bytes(), self.value.bytes()].bytes()
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

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.hashes(&self.inputs)
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.hashes(&self.outputs)
    }
    
    fn valuesum(&self, outs: &Vec<Output>) -> u64 {
        outs.iter().map(|o| o.value).sum()
    }
    
    fn hashes(&self, outs: &Vec<Output>) -> HashSet<Hash> {
        outs.iter()
            .map(|o| o.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Hash {
        self.inputs.bytes()
    }
}