use leptos::{prelude::*, *};
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use cfg_if::cfg_if;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse {
    status: String,
    message: String,
    version: String,
}

#[server(ApiEndpoint, "/api")]
#[cfg(feature = "ssr")]
pub async fn api_endpoint() -> Result<ApiResponse, ServerFnError> {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "Welcome to API v1".to_string(),
                version: "1.0".to_string(),
            })
        } else {
            use http::Request;
            let resp = Request::get("/api/v1")
                .body(())
                .map_err(|e| ServerFnError::Request(e.to_string()))?;
            
            let resp = crate::app::request_handler(resp).await?;
            let data = resp.into_body();
            
            serde_json::from_slice(&data)
                .map_err(|e| ServerFnError::Deserialization(e.to_string()))
        }
    }
}

#[component]
#[cfg(feature = "ssr")]
pub fn ApiPage() -> impl IntoView {
    let api_data = create_resource(
        || (),
        |_| async move { api_endpoint().await }
    );

    let api_content = move || {
        match api_data.get() {
            None => "Loading...".to_string(),
            Some(Ok(data)) => serde_json::to_string_pretty(&data)
                .unwrap_or_else(|_| "Error serializing data".to_string()),
            Some(Err(e)) => format!("Error: {}", e)
        }
    };

    let loading_view = view! {
        <div class="bg-gray-100 rounded-lg p-6">
            <div class="animate-pulse">"Loading..."</div>
        </div>
    };

    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">"API v1"</h1>
            // <Suspense
            //     fallback=loading_view
            // >
            //     <div class="bg-gray-100 rounded-lg p-6">
            //         <pre class="whitespace-pre-wrap font-mono text-sm">
            //             {api_content}
            //         </pre>
            //     </div>
            // </Suspense>
        </div>
    }
}
