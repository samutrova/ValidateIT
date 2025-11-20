use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FiscalCodeResponse {
    pub valid: bool,
    pub r#type: String,
    pub message: String,
    pub checks: FiscalCodeChecks
}

#[derive(Serialize, Deserialize)]
pub struct FiscalCodeChecks {
    pub format: bool,
    pub checksum: bool
}
