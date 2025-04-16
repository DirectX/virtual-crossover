use serde::{de::DeserializeOwned, Serialize};

pub mod models;

// use crate::api::ApiResponse;
// use leptos::*;
// use http::StatusCode;
// use leptos::ServerFnError;

// #[server(GetApiData, "/api/v1")]
// pub async fn get_api_data() -> Result<ApiResponse, ServerFnError> {
//     Ok(ApiResponse {
//         status: "success".to_string(),
//         message: "API is working".to_string(),
//     })
// }

// #[server(PostApiData, "/api/v1")]
// pub async fn post_api_data(data: String) -> Result<ApiResponse, ServerFnError> {
//     Ok(ApiResponse {
//         status: "success".to_string(),
//         message: format!("Received data: {}", data),
//     })
// }

// #[server(PutApiData, "/api/v1")]
// pub async fn put_api_data(id: String, data: String) -> Result<ApiResponse, ServerFnError> {
//     Ok(ApiResponse {
//         status: "success".to_string(),
//         message: format!("Updated id {} with data: {}", id, data),
//     })
// }

// #[server(DeleteApiData, "/api/v1")]
// pub async fn delete_api_data(id: String) -> Result<ApiResponse, ServerFnError> {
//     Ok(ApiResponse {
//         status: "success".to_string(),
//         message: format!("Deleted id: {}", id),
//     })
// }

pub fn story(path: &str) -> String {
    format!("https://node-hnapi.herokuapp.com/{path}")
}

pub fn user(path: &str) -> String {
    format!("https://hacker-news.firebaseio.com/v0/user/{path}.json")
}

#[cfg(not(feature = "ssr"))]
pub fn fetch_api<T>(
    path: &str,
) -> impl std::future::Future<Output = Option<T>> + Send + '_
where
    T: Serialize + DeserializeOwned,
{
    use leptos::prelude::on_cleanup;
    use send_wrapper::SendWrapper;

    SendWrapper::new(async move {
        let abort_controller =
            SendWrapper::new(web_sys::AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });

        gloo_net::http::Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| log::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(path: &str) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    reqwest::get(path)
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .json()
        .await
        .ok()
}
