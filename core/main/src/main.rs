use parsing::*;
use types::*;


// from SmartHelper/core:
// test at: ../tests/basic_mapping_slots/contracts_/ast_compact_json.json

fn main() {
    // Load the JSON source
    let source_unit: SourceUnit = load_json_file("../tests/basic_mapping_slots/contracts_/ast_compact_json_city2.json");
    let mut variables_dict = VariablesDict::new();

    for node in source_unit.nodes {
        if let Node::ContractDefinition { name, nodes } = node {
            println!("Contract: {}", name);

            // Initialize current slot and offset
            let mut current_slot: usize = 0;
            let mut current_offset: usize = 0;

            for contract_node in nodes {
                if let Node::VariableDeclaration { name: var_name, type_name } = contract_node {
                    if let Some(type_name) = type_name {
                        // Process the variable
                        let variable = process_variable(
                            var_name.clone(),
                            &type_name,
                            &mut current_offset,
                            &mut current_slot,
                            &var_name,
                            &var_name,
                        );

                        variables_dict.insert(var_name.clone(), variable);
                    }
                }
            }
        }
    }

    let json_output = to_string_pretty(&variables_dict).expect("Failed to serialize"); // this is just the template of the storage layout, of course we need MockData to seee where it will be stored
    println!("{}", json_output);
}



fn process_variable(
    name: String,
    type_name: &TypeName,
    current_offset: &mut usize,
    current_slot: &mut usize,
    path: &str,
    raw_path: &str,
) -> Variable {
    match type_name {
        // Handle elementary types (e.g., uint256, address)
        TypeName::ElementaryTypeName { name: var_type } => {
            let solidity_type = parse_solidity_type(var_type);
            let size = size_of_type(&solidity_type);

            // Check if the variable fits in the current slot
            if *current_offset + size > 256 {
                // Move to the next slot if it doesn't fit
                *current_slot += 1;
                *current_offset = 0;
            }

            let variable = Variable {
                name: Some(name),
                var_type: var_type.clone(),
                key: None,
                values: None,
                offset: *current_offset,
                slot: *current_slot,
                size,
                path: path.to_string(),
                raw_path: raw_path.to_string(),
                length: None, // Not applicable for elementary types
            };

            // Update the offset for the next variable
            *current_offset += size;
            variable
        }
        // Handle mappings
        TypeName::Mapping { key_type, value_type } => {
            *current_slot += 1; // Move to the next available slot for the mapping value
            *current_offset = 0;

            let mut intern_current_slot = 0u64 as usize;
            let mut intern_current_offset = 0u64 as usize;

            let value_variable = process_variable(
                "value".to_string(),
                value_type,
                &mut intern_current_slot,
                &mut intern_current_offset,
                &format!("{}.value", path),
                &format!("{}.value", raw_path),
            );

            // Compute keccak256(slot . key) for the mapping storage
            // let s = *current_slot;
            // let mut concatenated = Vec::new();
            // concatenated.extend_from_slice(&s.to_be_bytes());
            // concatenated.extend_from_slice(&keccak256(&key_info.slot.to_be_bytes()));
            // let computed_slot = keccak256(&concatenated);

            // Mappings always occupy a full slot
            let key_info = KeyInfo {
                var_type: format!("{}", &extract_type_name(key_type)),
                offset: 0,
                slot: 0, // without the value of a key we can't compute the key address
                size: 256, // Mapping keys always occupy 32 bytes
                path: format!("{}.key", path),
                raw_path: format!("{}.key", raw_path),
            };

            

            let mut values = VariablesDict::new();
            values.insert("value".to_string(), value_variable);

            Variable {
                name: Some(name),
                var_type: "mapping".to_string(),
                key: Some(key_info),
                values: Some(values),
                offset: 0,
                slot: *current_slot,
                size: 256,
                path: path.to_string(),
                raw_path: raw_path.to_string(),
                length: None, // Not applicable for mappings
            }
        }
        // Handle arrays (both fixed and dynamic)
        TypeName::ArrayTypeName { base_type, length } => {
            let element_type = parse_solidity_type(&extract_type_name(base_type));
            let element_size = size_of_type(&element_type);
            
            *current_slot += 1; // Arrays start in a new slot
            *current_offset = 0;

            let mut values = VariablesDict::new();
            
            // TODO: maybe for fixed array if there are a BIG number allocated, like 100000, we should just create an offset to update current slot, instead of mocking with internal values ?
            if let Some(len) = length.as_ref().and_then(|l| l.as_usize()) {

                let init_path = *current_slot;

                for i in 0..len {
                    let mut variable = process_variable(
                        format!("element_{}", i),
                        base_type,
                        current_offset,
                        current_slot,
                        "",
                        &format!("{}.element_{}", raw_path, i),
                    );

                    let mut computed_path = format!("{} + {} (offset: {})", init_path, variable.slot - init_path, variable.offset);
                    variable.path = computed_path;

                    values.insert(
                        format!("element_{}", i),
                        variable
                    );
                }

                // at the end: items per slot = 32 / element_size if we follow the documentation https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html
                let array_variable = Variable {
                    name: Some(name),
                    var_type: "array".to_string(),
                    key: None,
                    values: Some(values),
                    offset: 0,
                    slot: *current_slot,
                    size: element_size * len,
                    path: path.to_string(),
                    raw_path: raw_path.to_string(),
                    length: Some(len),
                };

                array_variable

            } else {
                // Dynamic array: compute the starting slot for elements
                let pointer_slot = *current_slot; // Pointer slot for the array
                let element_start_slot_hash = keccak256(&(pointer_slot as u64).to_be_bytes());
                // TODO: PROBLEME CAR CA DEVRAIT ETRE SUR 256 BITS ET PAS 64 LE INTERN_CURRENT_SLOT
                let mut intern_current_slot = usize::from_be_bytes(element_start_slot_hash[..8].try_into().unwrap()) as usize;
                let mut intern_current_offset = 0u64 as usize;

                values.insert(
                    "element".to_string(),
                    process_variable(
                        "element".to_string(),
                        base_type,
                        &mut intern_current_offset,
                        &mut intern_current_slot,
                        &format!("Keccak256({})", pointer_slot),
                        &format!("{}.element", raw_path),
                    ),
                );

                let array_variable = Variable {
                    name: Some(name),
                    var_type: "array".to_string(),
                    key: None,
                    values: Some(values),
                    offset: 0,
                    slot: pointer_slot, // Slot where the array's metadata is stored
                    size: 256,          // Size of the pointer
                    path: path.to_string(),
                    raw_path: raw_path.to_string(),
                    length: None,
                };

                array_variable
            }
        }
    }
}


use sha3::{Digest, Keccak256};

fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}


// used to unpack the type name
pub fn extract_type_name(type_name: &TypeName) -> String {
    match type_name {
        TypeName::ElementaryTypeName { name } => name.clone(),
        _ => "unknown".to_string(),
    }
}

// used to debug
#[allow(dead_code)]
fn print_type_recursively(type_name: &TypeName, depth: usize) {
    let indent = "  ".repeat(depth);

    match type_name {
        TypeName::ElementaryTypeName { name } => {
            println!("{}Type: {}", indent, name);
        }
        TypeName::Mapping { key_type, value_type } => {
            println!("{}Mapping:", indent);
            println!("{}  Key Type:", indent);
            print_type_recursively(key_type, depth + 2);
            println!("{}  Value Type:", indent);
            print_type_recursively(value_type, depth + 2);
        }
        TypeName::ArrayTypeName { base_type , length: _ } => {
            println!("{}Array of:", indent);
            print_type_recursively(base_type, depth + 1);
        }
    }
}
