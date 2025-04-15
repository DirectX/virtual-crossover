use crate::api::ApiResponse;
use leptos::*;
use http::StatusCode;
use leptos::ServerFnError;

#[server(GetApiData, "/api/v1")]
pub async fn get_api_data() -> Result<ApiResponse, ServerFnError> {
    Ok(ApiResponse {
        status: "success".to_string(),
        message: "API is working".to_string(),
    })
}

#[server(PostApiData, "/api/v1")]
pub async fn post_api_data(data: String) -> Result<ApiResponse, ServerFnError> {
    Ok(ApiResponse {
        status: "success".to_string(),
        message: format!("Received data: {}", data),
    })
}

#[server(PutApiData, "/api/v1")]
pub async fn put_api_data(id: String, data: String) -> Result<ApiResponse, ServerFnError> {
    Ok(ApiResponse {
        status: "success".to_string(),
        message: format!("Updated id {} with data: {}", id, data),
    })
}

#[server(DeleteApiData, "/api/v1")]
pub async fn delete_api_data(id: String) -> Result<ApiResponse, ServerFnError> {
    Ok(ApiResponse {
        status: "success".to_string(),
        message: format!("Deleted id: {}", id),
    })
}