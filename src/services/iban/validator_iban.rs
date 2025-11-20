use std::str::from_utf8;

use actix_web::http::StatusCode;

use crate::models::iban::{IbanChecks, IbanResponse};

const CIN_ODD_VAL: [u8; 36] = [
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21,
    2, 4, 18, 20, 11, 3, 6, 8, 12, 14,
    16, 10, 22, 25, 24, 23,
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21
];

const CIN_EVEN_VAL: [u8; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
];

pub struct ValidatorIban;

impl ValidatorIban {
    pub async fn validate(iban: &str) -> Result<IbanResponse, (String, StatusCode)> {
        let clean = iban.chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();

        let country = &clean[0..=1];
        if let Some(expected_len) = get_iban_length(country) {
            if clean.len() != expected_len {
                return Ok(IbanResponse {
                    valid: false,
                    r#type: "iban".to_string(),
                    message: format!("IBAN length must be {}", expected_len),
                    checks: IbanChecks {
                        format: false,
                        country_code: false,
                        checksum: false,
                        cin: false
                    }
                });
            }
        } else {
            return Ok(IbanResponse {
                valid: false,
                r#type: "iban".to_string(),
                message: "Invalid country code".to_string(),
                checks: IbanChecks {
                    format: true,
                    country_code: false,
                    checksum: false,
                    cin: false
                }
            });
        }

        let rearranged = format!("{}{}", &clean[4..], &clean[..4]);

        // Convertire lettere in numeri
        let numeric = rearranged.chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    c.to_string()
                } else {
                    ((c as u32) - ('A' as u32) + 10).to_string()
                }
            })
            .collect::<String>();

        if mod97_large(&numeric)? != 1 {
            return Ok(IbanResponse {
                valid: false,
                r#type: "iban".to_string(),
                message: "Invalid checksum".to_string(),
                checks: IbanChecks {
                    format: true,
                    country_code: true,
                    checksum: false,
                    cin: false
                }
            });
        }

        if country == "IT" {
            let calculated_cin = calculate_cin(&clean[5..]);
            let cin = &clean[4..=4];
            
            if calculated_cin.to_string() != cin {
                return Ok(IbanResponse {
                    valid: false,
                    r#type: "iban".to_string(),
                    message: "Invalid CIN".to_string(),
                    checks: IbanChecks {
                        format: true,
                        country_code: true,
                        checksum: true,
                        cin: false
                    }
                });
            } else {
                return Ok(IbanResponse {
                    valid: true,
                    r#type: "iban".to_string(),
                    message: "Invalid CIN".to_string(),
                    checks: IbanChecks {
                        format: true,
                        country_code: true,
                        checksum: true,
                        cin: true
                    }
                });
            }
        }

        Ok(IbanResponse {
            valid: true,
            r#type: "iban".to_string(),
            message: "IBAN is valid".to_string(),
            checks: IbanChecks {
                format: true,
                country_code: true,
                checksum: true,
                cin: false
            }
        })
    }
}

fn calculate_cin(bban: &str) -> char {
    let mut sum = 0u32;
    
    for (i, c) in bban.chars().enumerate() {
        let idx = if c.is_ascii_digit() {
            26 + c.to_digit(10).unwrap() as usize
        } else {
            (c as usize) - ('A' as usize)
        };
        
        let value = if i % 2 == 0 {
            CIN_ODD_VAL[idx]
        } else {
            CIN_EVEN_VAL[idx]
        };
        
        sum += value as u32;
    }
    
    let remainder = (sum % 26) as usize;
    ((remainder as u8) + b'A') as char
}

fn mod97_large(num_str: &str) -> Result<u32, (String, StatusCode)> {
    let mut remainder: u64 = 0;

    for chunk in num_str.as_bytes().chunks(9) {
        let chunk_str = match from_utf8(chunk) {
            Ok(s) => s,
            Err(err) => return Err((format!("Error decoding chunk: {}", err), StatusCode::INTERNAL_SERVER_ERROR))
        };
        let combined = format!("{}{}", remainder, chunk_str);
        let value = match combined.parse::<u64>() {
            Ok(v) => v,
            Err(err) => return Err((format!("Error parsing chunk: {}", err), StatusCode::INTERNAL_SERVER_ERROR))
        };
        
        remainder = value % 97;
    }

    Ok(remainder as u32)
}

fn get_iban_length(contry: &str) -> Option<usize> {
    match contry {
        "IT" | "FR" => Some(27),
        "DE" | "GB" => Some(22),
        "ES" => Some(24),
        "NL" => Some(18),
        "CH" => Some(21),
        "AT" => Some(20),
        "BE" => Some(16),
        "PT" => Some(25),
        _ => None
    }
}
