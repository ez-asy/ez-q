use actix_web::{error::InternalError, web::JsonConfig, HttpResponse};

pub fn create_json_config() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _req| {
        // create custom error response
        InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
    })
}
