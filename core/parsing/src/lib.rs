use serde::{Deserialize, Serialize};
pub use serde_json::to_string_pretty;

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceUnit {
    pub nodes: Vec<Node>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayLength {
    value: Option<String>, // The length value as a string
}

impl ArrayLength {
    pub fn as_usize(&self) -> Option<usize> {
        self.value.as_ref().and_then(|v| v.parse::<usize>().ok())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "nodeType", rename_all = "PascalCase")]
pub enum Node {
    ContractDefinition {
        name: String,   
        nodes: Vec<Node>,
    },
    VariableDeclaration {
        name: String,
        #[serde(rename = "typeName")]
        type_name: Option<TypeName>,
    },
    #[serde(other)] // must have to ignore other useless things
    Ignored,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "nodeType", rename_all = "PascalCase")]
pub enum TypeName {
    ElementaryTypeName {
        name: String, // basic types (uint, address, etc.)
    },
    Mapping {
        #[serde(rename = "keyType")]
        key_type: Box<TypeName>, 
        #[serde(rename = "valueType")]
        value_type: Box<TypeName>, 
    },
    ArrayTypeName {
        #[serde(rename = "baseType")]
        base_type: Box<TypeName>, 
        length: Option<ArrayLength>
    },
}

pub fn load_json_file(file_path: &str) -> SourceUnit {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read JSON file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON")
}