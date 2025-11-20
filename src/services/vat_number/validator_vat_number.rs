use actix_web::http::StatusCode;
use regex::Regex;

use crate::models::vat_number::{VatNumberChecks, VatNumberResponse};

pub struct ValidatorVatNumber;

impl ValidatorVatNumber {
    pub async fn validate(vat_number: &str) -> Result<VatNumberResponse, (String, StatusCode)> {
        let pattern = 
            match Regex::new("^[0-9]{11}$") {
                Ok(pattern) => pattern,
                Err(err) => return Err(
                    (format!("Regex error with pattern matching VAT number: {}", err), StatusCode::INTERNAL_SERVER_ERROR)
                )
            };
        if !pattern.is_match(vat_number) {
            return Err(("Invalid VAT number format".to_string(), StatusCode::BAD_REQUEST));
        }

        if vat_number.len() != 11 {
            return Ok(VatNumberResponse {
                valid: false,
                r#type: "vat_number".to_string(),
                message: "VAT Number length must be 11".to_string(),
                checks: VatNumberChecks {
                    format: false,
                    office_code: false,
                    checksum: false
                }
            });
        }

        if !validate_office_code(vat_number)? {
            return Ok(VatNumberResponse {
                valid: false,
                r#type: "vat_number".to_string(),
                message: "Invalid office code".to_string(),
                checks: VatNumberChecks {
                    format: true,
                    office_code: false,
                    checksum: false
                }
            });
        }

        let calculated_check_digit = calculate_check_digit(&vat_number[0..=9])?;
        let check_digit = &vat_number[10..=10];

        if calculated_check_digit.to_string() != check_digit {
            return Ok(
                VatNumberResponse {
                    valid: false,
                    r#type: "vat_number".to_string(),
                    message: "Invalid check digit".to_string(),
                    checks: VatNumberChecks {
                        format: true,
                        office_code: true,
                        checksum: false
                    }
                }
            );
        }

        Ok(VatNumberResponse {
            valid: true,
            r#type: "vat_number".to_string(),
            message: "VAT Number is valid".to_string(),
            checks: VatNumberChecks {
                format: true,
                office_code: true,
                checksum: true
            }
        })
    }
}

fn calculate_check_digit(vat_number_10: &str) -> Result<u32, (String, StatusCode)> {
    let digits = vat_number_10.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
        
    let mut sum: u32 = 0;
        
    for i in 0..10 {
        let mut digit = digits[i];
            
        // Posizioni PARI (indice dispari, perché 1-indexed)
        if i % 2 == 1 {
            digit *= 2;
                
            if digit > 9 {
                digit = (digit / 10) + (digit % 10);
            }
        }
            
        sum += digit;
    }
        
    let check_digit = (10 - (sum % 10)) % 10;
        
    // Verifica con l'11esima cifra
    Ok(check_digit)
}

fn validate_office_code(vat_number: &str) -> Result<bool, (String, StatusCode)> {
    let office_code = match vat_number[7..=9].parse::<u32>() {
        Ok(office_code) => office_code,
        Err(err) => return Err((format!("Error parsing office code: {}", err), StatusCode::INTERNAL_SERVER_ERROR))
    };

    Ok(
        (office_code >= 1 && office_code <= 100) ||
        office_code == 120 ||
        office_code == 121 ||
        office_code == 888 ||
        office_code == 999
    )
}
