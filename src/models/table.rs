use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnDefinition {
    pub id: Option<i32>,
    pub column_name: String,
    pub data_type: String,
    pub is_required: bool,
    pub min_val: Option<f64>,
    pub max_val: Option<f64>,
    pub references: Option<String>,
}

#[derive(Debug. Serialize, Deserialize, Clone)]
pub struct DefineTable {
    pub table_name: String,
    pub column: Option<Vec<ColumnDefinition>>,
    pub permission: Option<Vec<i16>>,
}

pub struct Table