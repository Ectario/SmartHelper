use parsing::*;
use types::*;


// from SmartHelper/core:
// test at: ../tests/basic_mapping_slots/contracts_/ast_compact_json.json

fn main() {
    let source_unit: SourceUnit = load_json_file("../tests/basic_mapping_slots/contracts_/ast_compact_json_city2.json");
    let mut variables_dict = VariablesDict::new();

    for node in source_unit.nodes {
        if let Node::ContractDefinition { name, nodes } = node {
            println!("Contract: {}", name);

            for contract_node in nodes {
                if let Node::VariableDeclaration { name: var_name, type_name } = contract_node {
                    if let Some(type_name) = type_name {
                        let variable = process_variable(var_name.clone(), &type_name, 0, 0, &var_name, &var_name);
                        variables_dict.insert(var_name.clone(), variable);
                    }
                }
            }
        }
    }

    // println!("Variables: {:#?}", variables_dict);
    let json_output = to_string_pretty(&variables_dict).expect("Failed to serialize");
    println!("{}", json_output);
}

fn process_variable(name: String, type_name: &TypeName, offset: usize, slot: usize, path: &str, raw_path: &str) -> Variable {
    match type_name {
        // Handle elementary types (e.g., uint256, address)
        TypeName::ElementaryTypeName { name: var_type } => {
            let solidity_type = parse_solidity_type(var_type);

            Variable {
                name: Some(name),
                var_type: var_type.clone(),
                key: None,
                values: None,
                offset,
                slot,
                size: size_of_type(&solidity_type),
                path: path.to_string(),
                raw_path: raw_path.to_string(),
                length: None, // Not applicable for elementary types
            }
        }
        // Handle mappings
        TypeName::Mapping { key_type, value_type } => {
            let key_info = KeyInfo {
                var_type: format!("{}", &extract_type_name(key_type)),
                offset: 0,
                slot,
                size: 32, // Mapping keys always occupy 32 bytes
                path: format!("{}.key", path),
                raw_path: format!("{}.key", raw_path),
            };

            // Recursive call for the mapping's value
            let value_variable = process_variable("value".to_string(), value_type, 0, slot + 1, &format!("{}.value", path), &format!("{}.value", raw_path));

            let mut values = VariablesDict::new();
            values.insert("value".to_string(), value_variable);

            Variable {
                name: Some(name),
                var_type: "mapping".to_string(),
                key: Some(key_info),
                values: Some(values),
                offset,
                slot,
                size: 32, // Mappings themselves point to a 32-byte slot
                path: path.to_string(),
                raw_path: raw_path.to_string(),
                length: None, // Not applicable for mappings
            }
        }
        // Handle arrays (both fixed and dynamic)
        TypeName::ArrayTypeName { base_type, length } => {
            let element_type = parse_solidity_type(&extract_type_name(base_type));
            let mut values = VariablesDict::new();

            // Add a placeholder for an example element in the array
            values.insert(
                "element".to_string(),
                process_variable("element".to_string(), base_type, offset, slot, &format!("{}.element", path), &format!("{}.element", raw_path)),
            );

            let total_size = if let Some(len) = length.as_ref().and_then(|l| l.as_usize()) {
                // Fixed array: calculate total size as element size * length
                size_of_type(&element_type) * len
            } else {
                // Dynamic array: size is always 32 bytes for a pointer
                32
            };

            Variable {
                name: Some(name),
                var_type: "array".to_string(),
                key: None,
                values: Some(values), // Store nested structure for array elements
                offset,
                slot,
                size: total_size,
                path: path.to_string(),
                raw_path: raw_path.to_string(),
                length: length.as_ref().and_then(|l| l.as_usize()), // Length is stored only for fixed arrays
            }
        }
    }
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
