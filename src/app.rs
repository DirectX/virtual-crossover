use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

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
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment_count = move |_| *set_count.write() += 1;

    let (brightness, set_brightness) = signal(50);
    let (volume, set_volume) = signal(70);
    let (feature_a, set_feature_a) = signal(false);
    let (feature_b, set_feature_b) = signal(true);
    let (mode, set_mode) = signal("Standard".to_string());

    view! {
        <div class="container mx-auto">
            <div class="mx-auto flex max-w-sm items-center my-4 gap-x-4 rounded-xl bg-white p-6 shadow-lg outline outline-black/5 dark:bg-slate-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">
                <img class="size-12 shrink-0" src="/assets/logo.svg" alt="Virtual Crossover Logo" />
                <div>
                    <div class="text-xl font-medium text-black dark:text-white">Virtual Crossover</div>
                    <p class="text-gray-500 dark:text-gray-400">AirPlay</p>
                </div>
            </div>

            <div class="border-b border-gray-200 bg-white px-4 py-5 sm:px-6">
                <h3 class="text-base font-semibold text-gray-900">label</h3>
            </div>
            <div class="md:columns-2">
                <div class="w-full aspect-video mb-4 p-4 bg-blue-500 text-white rounded-lg shadow">
                    <label>"Brightness:"</label>
                    // <input type="range" min=1 max=100 step=1 value=brightness on:change=move |e| set_brightness(&e.value_of) /> 
                </div>
                <div class="w-full aspect-video mb-4 p-4 bg-green-500 text-white rounded-lg shadow">
                    <label>"Volume:"</label>
                    // <input type="range" min=1 max=100 step=1 value=brightness on:change=move |e| set_brightness(&e.value_of) /> 
                </div>
            </div>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=increment_count>"Click Me: " {count}</button>
            // <div class="p-4 bg-blue-500 text-white rounded-lg shadow">
            //     "Hello from Tailwind!"
            // </div>
            // <div class="min-h-screen bg-gray-50 p-6">
            //     <div class="max-w-4xl mx-auto grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            //         <div class="bg-white rounded-2xl shadow p-4">
            //             <label class="block text-sm font-medium text-gray-700 mb-2">"Brightness"</label>
            //             <input
            //                 type="range"
            //                 min="0"
            //                 max="100"
            //                 class="w-full"
            //                 on:input=set_brightness
            //             />
            //             <p class="text-right text-sm text-gray-500 mt-1">{brightness)}</p>
            //         </div>
            //         <div class="bg-white rounded-2xl shadow p-4">
            //             <label class="block text-sm font-medium text-gray-700 mb-2">"Options"</label>
            //             <div class="space-y-2">
            //                 <label class="flex items-center space-x-2">
            //                     <input
            //                         type="checkbox"
            //                         checked=feature_a
            //                         on:change=move |e| set_feature_a(event_target_checked(&e))
            //                         class="form-checkbox text-blue-600"
            //                     />
            //                     <span>"Enable feature A"</span>
            //                 </label>
            //                 <label class="flex items-center space-x-2">
            //                     <input
            //                         type="checkbox"
            //                         checked=feature_b
            //                         on:change=move |e| set_feature_b(event_target_checked(&e))
            //                         class="form-checkbox text-blue-600"
            //                     />
            //                     <span>"Enable feature B"</span>
            //                 </label>
            //             </div>
            //         </div>
            //         <div class="bg-white rounded-2xl shadow p-4">
            //             <label class="block text-sm font-medium text-gray-700 mb-2">"Mode"</label>
            //             <select
            //                 class="w-full border-gray-300 rounded-lg shadow-sm"
            //                 on:change=move |e| set_mode(event_target_value(&e))
            //             >
            //                 <option value="Standard">"Standard"</option>
            //                 <option value="Advanced">"Advanced"</option>
            //                 <option value="Experimental">"Experimental"</option>
            //             </select>
            //             <p class="text-sm text-gray-500 mt-1">"Selected: " {move || mode()}</p>
            //         </div>
            //         <div class="bg-white rounded-2xl shadow p-4">
            //             <label class="block text-sm font-medium text-gray-700 mb-2">"Volume"</label>
            //             <input
            //                 type="range"
            //                 min="0"
            //                 max="100"
            //                 class="w-full"
            //                 on:input=move |e| {
            //                     let val = event_target_value(&e).parse().unwrap_or(0);
            //                     set_volume(val);
            //                 }
            //             />
            //             <p class="text-right text-sm text-gray-500 mt-1">{move || format!("{}%", volume())}</p>
            //         </div>
            //     </div>
            // </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
