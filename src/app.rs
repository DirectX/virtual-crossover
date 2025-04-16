use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router}, OptionalParamSegment, ParamSegment, StaticSegment, WildcardSegment
};

use crate::{pages::{HomePage, NotFound, Story}};

#[component]
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
                // <Routes fallback=move || "Not found.">
                //     <Route path=StaticSegment("") view=HomePage/>
                //     <Route path=WildcardSegment("any") view=NotFound/>
                // </Routes>

                <FlatRoutes fallback=|| "Not found.">
                    // <Route path=(StaticSegment("users"), ParamSegment("id")) view=User/>
                    <Route path=(StaticSegment("stories"), ParamSegment("id")) view=Story/>
                    <Route path=OptionalParamSegment("stories") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </FlatRoutes>
            </main>
        </Router>
    }
}