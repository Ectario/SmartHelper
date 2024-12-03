use parsing::*;

// from SmartHelper/core:
// test at: ../tests/basic_mapping_slots/contracts_/ast_compact_json.json

fn main() {
    let source_unit: SourceUnit = load_json_file("../tests/basic_mapping_slots/contracts_/ast_compact_json.json");

    for node in source_unit.nodes {
        if let Node::ContractDefinition { name, nodes } = node {
            println!("Contract: {}", name);

            for contract_node in nodes {
                if let Node::VariableDeclaration { name: var_name, type_name } = contract_node {
                    if let Some(type_name) = type_name {
                        println!("  Variable: {}", var_name);
                        print_type_recursively(&type_name, 2);
                    }
                }
            }
        }
    }
}

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
        TypeName::ArrayTypeName { base_type } => {
            println!("{}Array of:", indent);
            print_type_recursively(base_type, depth + 1);
        }
    }
}
