use super::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Output {
    pub to_address: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_address.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}

impl Output {
    pub fn new(to_address: Address, value: u64) -> Self {
        assert!(value > 0);

        Self { to_address, value }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{{to_address: {}, value: {}}}",
            self.to_address, self.value
        )
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
    pub timestamp: u128,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{{hash: {}, timestamp: {}, inputs: {}, outputs: {}}}",
            hex::encode(self.hash()),
            self.timestamp,
            self.inputs
                .iter()
                .map(|i| i.to_string())
                .collect::<String>(),
            self.outputs
                .iter()
                .map(|o| o.to_string())
                .collect::<String>()
        )
    }
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            inputs: vec![],
            outputs: vec![],
            timestamp: now(),
        }
    }

    pub fn add_input(&mut self, value: Output) -> Vec<Output> {
        self.inputs.push(value);
        self.inputs.to_vec()
    }

    pub fn add_output(&mut self, value: Output) -> Vec<Output> {
        self.outputs.push(value);
        self.outputs.to_vec()
    }

    pub fn add_input_value(&mut self, to_address: Address, value: u64) -> Vec<Output> {
        assert!(value > 0);

        let input = Output::new(to_address, value);
        self.inputs.push(input);
        self.inputs.to_vec()
    }

    pub fn add_output_value(&mut self, to_address: Address, value: u64) -> Vec<Output> {
        assert!(value > 0);

        let output = Output::new(to_address, value);
        self.outputs.push(output);
        self.outputs.to_vec()
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn fee_value(&self) -> u64 {
        if self.is_coinbase() {
            0
        } else {
            self.input_value() - self.output_value()
        }
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let inputs = self
            .inputs
            .iter()
            .flat_map(|input| input.bytes())
            .collect::<Vec<u8>>();
        let outputs = self
            .outputs
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>();

        let mut bytes = vec![];
        bytes.extend(inputs);
        bytes.extend(outputs);
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes
    }
}
