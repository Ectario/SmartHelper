use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: Option<String>,         
    pub var_type: String,               // type variable (e.g., uint, mapping, etc.)
    pub key: Option<KeyInfo>,           // only for mappings: key type, offset, slot, size, etc..
    pub values: Option<VariablesDict>,  // only for mappings or arrays (this is children values)
    pub offset: usize,                  // offset in bits (offset in its slot)
    pub slot: usize,                    // slot number
    pub size: usize,                    // memory size in bits
    pub path: String,                   // Path for identifying the variable (string to describe keccak calculation of the slot)
    pub raw_path: String,               // Raw path of the variable (e.g., parent.value.key or parent.key.key or parent.value.value, etc..)
    pub length: Option<usize>,          // Length of the array (if it is an array, otherwise None)   
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub var_type: String, // key type (e.g., uint, address)
    pub offset: usize,    // offset in bits (offset in its slot)
    pub slot: usize,      // slot number
    pub size: usize,      // memory size in bits
    pub path: String,     // Path for identifying the variable (string to describe keccak calculation of the slot)
    pub raw_path: String, // Raw path of the variable (e.g., parent.value.key or parent.key.key or parent.value.value, etc..)
}

// Un dictionnaire de variables
pub type VariablesDict = HashMap<String, Variable>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolidityType {
    Uint(usize),          // uint8, uint16, ..., uint256
    Int(usize),           // int8, int16, ..., int256
    Address,           // address
    Bool,              // bool
    FixedBytes(u16),    // bytes1, bytes2, ..., bytes32
    DynamicBytes,      // bytes
    String,            // string
    FixedArray(Box<SolidityType>, usize), // fixed array
    DynamicArray(Box<SolidityType>),     // dynamic array
}

// get the size in bits of a Solidity Types
pub fn size_of_type(sol_type: &SolidityType) -> usize {
    match sol_type {
        SolidityType::Uint(size) | SolidityType::Int(size) => *size as usize,
        SolidityType::Address => 20*8, 
        SolidityType::Bool => 8,   
        SolidityType::FixedBytes(size) => *size as usize,
        SolidityType::DynamicBytes | SolidityType::String => 256,
        SolidityType::FixedArray(base_type, length) => size_of_type(base_type) * length,
        SolidityType::DynamicArray(_) => 256,
    }
}

pub fn parse_solidity_type(type_name: &str) -> SolidityType {
    match type_name {
        // Uint types (uint8, uint16, ..., uint256)
        t if t.starts_with("uint") => {
            let size = t[4..].parse::<usize>().unwrap_or(256);
            SolidityType::Uint(size)
        }
        // Int types (int8, int16, ..., int256)
        t if t.starts_with("int") => {
            let size = t[3..].parse::<usize>().unwrap_or(256);
            SolidityType::Int(size)
        }
        // Address type
        "address" => SolidityType::Address,
        // Boolean type
        "bool" => SolidityType::Bool,
        // Fixed-sized byte arrays (bytes1, bytes2, ..., bytes32)
        t if t.starts_with("bytes") && t.len() > 5 => {
            let size = t[5..].parse::<u16>().unwrap();
            SolidityType::FixedBytes(size*8)
        }
        // Dynamic bytes
        "bytes" => SolidityType::DynamicBytes,
        // String
        "string" => SolidityType::String,
        // Default: Unknown type
        _ => panic!("Unknown Solidity type: {}", type_name),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}
