use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use regex::Regex;

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
    let array_regex = Regex::new(r"(.+?)\[(\d*)\]$").unwrap();
    println!("type_name: {:?}", type_name);
    if let Some(captures) = array_regex.captures(type_name) {
        let base_type = &captures[1];
        let length = captures.get(2).map(|m| m.as_str());

        let parsed_base_type = parse_solidity_type(base_type);

        // fixed size array
        return if let Some(len) = length {
            if let Ok(len) = len.parse::<usize>() {
                SolidityType::FixedArray(Box::new(parsed_base_type), len)
            } else { // dynamic size array
                SolidityType::DynamicArray(Box::new(parsed_base_type))
            }
        } else { // weird case ?
            panic!("Weird case in parse_solidity_type with: {:?}", type_name)
        };
    }

    match type_name {
        t if t.starts_with("uint") => {
            let size = t[4..].parse::<usize>().unwrap_or(256);
            SolidityType::Uint(size)
        }
        t if t.starts_with("int") => {
            let size = t[3..].parse::<usize>().unwrap_or(256);
            SolidityType::Int(size)
        }
        "address" => SolidityType::Address,
        "bool" => SolidityType::Bool,
        t if t.starts_with("bytes") => {
            if let Ok(size) = t[5..].parse::<usize>() {
                SolidityType::FixedBytes(size as u16)
            } else {
                SolidityType::DynamicBytes
            }
        }
        "bytes" => SolidityType::DynamicBytes,
        "string" => SolidityType::String,
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
