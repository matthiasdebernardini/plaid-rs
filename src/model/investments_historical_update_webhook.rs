use serde::{Serialize, Deserialize};
use super::{PlaidError, WebhookEnvironmentValues};
///Fired after an asynchronous extraction on an investments account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsHistoricalUpdateWebhook {
    ///The number of canceled transactions reported since the last time this webhook was fired.
    pub canceled_investments_transactions: f64,
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///The number of new transactions reported since the last time this webhook was fired.
    pub new_investments_transactions: f64,
    ///`HISTORICAL_UPDATE`
    pub webhook_code: String,
    ///`INVESTMENTS_TRANSACTIONS`
    pub webhook_type: String,
}
impl std::fmt::Display for InvestmentsHistoricalUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
