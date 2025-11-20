use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IbanResponse {
    pub valid: bool,
    pub r#type: String,
    pub message: String,
    pub checks: IbanChecks
}

#[derive(Serialize, Deserialize)]
pub struct IbanChecks {
    pub format: bool,
    pub country_code: bool,
    pub checksum: bool,
    pub cin: bool
}
