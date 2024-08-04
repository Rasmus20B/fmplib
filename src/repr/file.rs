use crate::repr::component;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FmpFile {
    pub name: String,
    pub tables: HashMap<usize, component::FMComponentTable>,
    pub relationships: HashMap<usize, component::FMComponentRelationship>,
    pub layouts: HashMap<usize, component::FMComponentLayout>,
    pub value_lists: HashMap<usize, component::FMComponentValueList>,
    pub scripts: HashMap<usize, component::FMComponentScript>,
    pub table_occurrences: HashMap<usize, component::FMComponentTableOccurence>,
    pub tests: Vec<component::FMComponentTest>,
}

impl FmpFile {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            tables: HashMap::new(),
            relationships: HashMap::new(),
            layouts: HashMap::new(),
            value_lists: HashMap::new(),
            scripts: HashMap::new(),
            table_occurrences: HashMap::new(),
            tests: vec![],
        }
    }
}
