use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VatNumberResponse {
    pub valid: bool,
    pub r#type: String,
    pub message: String,
    pub checks: VatNumberChecks
}

#[derive(Serialize, Deserialize)]
pub struct VatNumberChecks {
    pub format: bool,
    pub office_code: bool,
    pub checksum: bool
}
