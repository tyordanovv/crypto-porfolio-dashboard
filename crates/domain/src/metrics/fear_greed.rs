use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FearGreedIndexData {
    pub value: String,
    pub classification: String,
    pub timestamp: String,
}