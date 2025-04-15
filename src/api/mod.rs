pub mod v1;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}