use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayPeriodDetailsPayFrequency(pub serde_json::Value);
