
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::script_engine::instructions::ScriptStep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FMComponentType {
    Table,
    Field,
    Layout,
    Script,
    TableOccurence,
    Relationship,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentField {
    pub data_type: String,
    pub field_description: String,
    pub field_name: String,
    pub field_type: String,
    pub created_by_account: String,
    pub created_by_user: String,
}
impl FMComponentField {
    pub fn new() -> Self {
        Self {
            data_type: String::new(),
            field_description: String::new(),
            field_name: String::new(),
            field_type: String::new(),
            created_by_account: String::new(),
            created_by_user: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTest {
    pub test_name: String,
    pub script: FMComponentScript,
    pub created_by_account: String,
    pub create_by_user: String,
    pub assertions: Vec<String>,
}

impl FMComponentTest {
    pub fn new() -> Self {
        Self {
            test_name: String::new(),
            script: FMComponentScript::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            assertions: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTable {
    pub table_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
    pub fields: HashMap<u16, FMComponentField>,
    pub init: bool,
}

impl FMComponentTable {
    pub fn new() -> Self {
        Self {
            table_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            fields: HashMap::new(),
            init: false
        }
    }

    pub fn new_init() -> Self {
        Self {
            table_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            fields: HashMap::new(),
            init: true
        }

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentScript {
    pub script_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
    pub arguments: Vec<String>,
    pub instructions: HashMap<usize, ScriptStep>,
} 

impl FMComponentScript {
    pub fn new() -> Self {
        Self {
            script_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            arguments: vec![],
            instructions: HashMap::new() 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTableOccurence {
    pub table_occurence_name: String,
    pub table_actual: u16,
    pub table_actual_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentTableOccurence {
    pub fn new() -> Self {
        Self {
            table_occurence_name: String::new(),
            table_actual: 0,
            table_actual_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentRelationship {
    pub table1: u16,
    pub table1_name: String,
    pub table2: u16,
    pub table2_name: String,
    pub comparison: u8,
}

impl FMComponentRelationship {
    pub fn new() -> Self {
        Self {
            table1: 0,
            table1_name: String::new(),
            table2: 0,
            table2_name: String::new(),
            comparison: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentLayout {
    pub layout_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentLayout {
    pub fn new() -> Self {
        Self {
            layout_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentValueList {
    pub list_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentValueList {
    pub fn new() -> Self {
        Self {
            list_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
        }
    }
}
