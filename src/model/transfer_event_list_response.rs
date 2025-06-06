use serde::{Serialize, Deserialize};
use super::TransferEvent;
///Defines the response schema for `/transfer/event/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferEventListResponse {
    ///Whether there are more events to be pulled from the endpoint that have not already been returned
    pub has_more: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transfer_events: Vec<TransferEvent>,
}
impl std::fmt::Display for TransferEventListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
