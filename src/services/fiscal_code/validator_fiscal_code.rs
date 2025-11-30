use actix_web::http::StatusCode;
use regex::Regex;

use crate::models::fiscal_code::{FiscalCodeChecks, FiscalCodeResponse};

const ODD_VALUES: [u8; 36] = [
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21,  // A-J
    2, 4, 18, 20, 11, 3, 6, 8, 12, 14,  // K-T
    16, 10, 22, 25, 24, 23,              // U-Z
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21   // 0-9
];

const EVEN_VALUES: [u8; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,       // A-J
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // K-T
    20, 21, 22, 23, 24, 25,              // U-Z
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9        // 0-9
];

const CHECK_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z'
];

const OMOCODIA_TO_DIGIT: &[(&str, char)] = &[
    ("L", '0'), ("M", '1'), ("N", '2'), ("P", '3'), ("Q", '4'),
    ("R", '5'), ("S", '6'), ("T", '7'), ("U", '8'), ("V", '9'),
];

const OMOCODIA_POSITIONS: [usize; 7] = [6, 7, 9, 10, 12, 13, 14];

pub struct ValidatorFiscalCode;

impl ValidatorFiscalCode {
    pub async fn validate(fiscal_code: &str) -> Result<FiscalCodeResponse, (String, StatusCode)> {
        let fiscal_code = fiscal_code.to_uppercase();
        
        let pattern = 
            match Regex::new("^[A-Z]{6}[0-9LMNPQRSTUV]{2}[A-Z][0-9LMNPQRSTUV]{2}[A-Z][0-9LMNPQRSTUV]{3}[A-Z]$") {
                Ok(pattern) => pattern,
                Err(err) => return Err(
                    (
                        format!("Regex error with pattern matching fiscal code: {}", err), 
                        StatusCode::INTERNAL_SERVER_ERROR
                    )
                )
            };
        if !pattern.is_match(&fiscal_code) {
            return Ok(FiscalCodeResponse {
                valid: false,
                r#type: "fiscal_code".to_string(),
                message: "Invalid fiscal code format".to_string(),
                checks: FiscalCodeChecks {
                    format: false,
                    checksum: false
                }
            });
        }

        let fiscal_code = fiscal_code.chars().enumerate().map(|(i, c)| {
            if OMOCODIA_POSITIONS.contains(&i) && c.is_alphabetic() {
                OMOCODIA_TO_DIGIT.iter()
                    .find(|&&(lett, _)| lett == c.to_string())
                    .map(|&(_, digit)| digit)
                    .unwrap_or(c)
            } else {
                c
            }
        }).collect::<String>();

        let fiscal_code_15 = &fiscal_code[..=14];
        let check_digit = fiscal_code.as_bytes()[15] as char;

        let calculated_check_digit = calculate_check_digit(fiscal_code_15)?;

        if calculated_check_digit != check_digit {
            return Ok(FiscalCodeResponse {
                valid: false,
                r#type: "fiscal_code".to_string(),
                message: "Fiscal Code checksum is not valid".to_string(),
                checks: FiscalCodeChecks {
                    format: true,
                    checksum: false
                }
            });
        }

        Ok(FiscalCodeResponse {
            valid: true,
            r#type: "fiscal_code".to_string(),
            message: "Fiscal Code is valid".to_string(),
            checks: FiscalCodeChecks {
                format: true,
                checksum: true
            }
        })
    }
}

fn calculate_check_digit(fiscal_code_15: &str) -> Result<char, (String, StatusCode)> {
    let mut sum: u32 = 0;

    for (i, c) in fiscal_code_15.chars().enumerate() {
        let value = if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as usize;

            if i % 2 == 0 { // Posizione dispari
                ODD_VALUES[26 + digit]
            } else {
                EVEN_VALUES[26 + digit]
            }
        } else {
            let letter_idx = (c as usize) - ('A' as usize);

            if i % 2 == 0 { // Posizione dispari
                ODD_VALUES[letter_idx]
            } else {
                EVEN_VALUES[letter_idx]
            }
        };

        sum += value as u32;
    }

    let remainder = (sum % 26) as usize;
    Ok(CHECK_LETTERS[remainder])
}
