use actix_web::middleware::{NormalizePath, TrailingSlash};

pub fn create_normalise_mw() -> NormalizePath {
    NormalizePath::new(TrailingSlash::Always)
}
