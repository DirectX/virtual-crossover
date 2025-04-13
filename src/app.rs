use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[cfg(feature = "ssr")]
use crate::pages::{api::ApiPage, HomePage, NotFound};

#[component]
#[cfg(feature = "ssr")]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/virtual-crossover.css"/>

        // sets the document title
        <Title text="Virtual Crossover"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path="/api/v1" view=ApiPage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}