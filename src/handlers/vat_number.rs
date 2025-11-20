use actix_web::{HttpResponse, get, web};

use crate::{handlers::ApiError, services::vat_number::validator_vat_number::ValidatorVatNumber};

#[get("/{vat_number}/validate")]
pub async fn validate_vat_number(vat_number: web::Path<String>) -> actix_web::Result<HttpResponse> {
    match ValidatorVatNumber::validate(&vat_number).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Ok(
            HttpResponse::build(err.1).json(ApiError {
                message: err.0,
                status_code: err.1.as_u16(),
                timestamp: chrono::Utc::now().to_rfc3339()
            })
        )
    }
}
