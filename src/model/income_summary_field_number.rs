use serde::{Serialize, Deserialize};
use super::VerificationStatus;
///Field number for income summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeSummaryFieldNumber {
    ///The value of the field.
    pub value: f64,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
impl std::fmt::Display for IncomeSummaryFieldNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
