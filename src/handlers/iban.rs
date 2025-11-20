use actix_web::{HttpResponse, get, web};

use crate::{handlers::ApiError, services::iban::validator_iban::ValidatorIban};

#[get("/{iban}/validate")]
pub async fn validate_iban(iban: web::Path<String>) -> actix_web::Result<HttpResponse> {
    match ValidatorIban::validate(&iban).await {
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
