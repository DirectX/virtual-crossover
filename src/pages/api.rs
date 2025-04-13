use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse {
    status: String,
    message: String,
    version: String,
}

#[server(ApiEndpoint, "/api")]
pub async fn api_endpoint() -> Result<ApiResponse, ServerFnError> {
    Ok(ApiResponse {
        status: "success".to_string(),
        message: "Welcome to API v1".to_string(),
        version: "1.0".to_string(),
    })
}

#[component]
pub fn ApiPage() -> impl IntoView {
    let api_data = create_resource(
        || (),
        |_| async move { api_endpoint().await.unwrap_or_else(|_| ApiResponse {
            status: "error".to_string(),
            message: "Failed to fetch data".to_string(),
            version: "1.0".to_string(),
        })}
    );

    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">"API v1"</h1>
            <div class="bg-gray-100 rounded-lg p-6">
                <pre class="whitespace-pre-wrap font-mono text-sm">
                    {move || {
                        api_data.get().map(|data| {
                            serde_json::to_string_pretty(&data)
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        }).unwrap_or_else(|| "Loading...".to_string())
                    }}
                </pre>
            </div>
        </div>
    }
}