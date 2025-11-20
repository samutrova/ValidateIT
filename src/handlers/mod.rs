use serde::{Deserialize, Serialize};

pub mod fiscal_code;
pub mod vat_number;
pub mod iban;

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub status_code: u16,
    pub timestamp: String
}
