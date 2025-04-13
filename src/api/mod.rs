pub mod v1;

#[cfg(feature = "ssr")]
pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    v1::config(cfg);
}