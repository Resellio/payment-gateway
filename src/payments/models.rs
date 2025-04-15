use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessPaymentRequest {
    pub(super) amount: f64,
    pub(super) currency: String,
    pub(super) card_number: String,
    pub(super) card_expiry: String,
    pub(super) cvv: String,
    pub(super) force_error: bool,
}
