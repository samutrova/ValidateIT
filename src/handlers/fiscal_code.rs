use actix_web::{HttpResponse, get, web};

use crate::{handlers::ApiError, services::fiscal_code::validator_fiscal_code::ValidatorFiscalCode};

#[get("/{fiscal_code}/validate")]
pub async fn validate_fiscal_code(fiscal_code: web::Path<String>) -> actix_web::Result<HttpResponse> {
    match ValidatorFiscalCode::validate(&fiscal_code).await {
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
