pub mod v1;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    v1::config(cfg);
}