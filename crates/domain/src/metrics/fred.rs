use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FredIndexData {
    pub series_id: String,
    pub value: f64,
    pub date: String,
}