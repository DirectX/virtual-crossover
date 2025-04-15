use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query_map};
use crate::api;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let (volume, set_volume) = signal(70);

    let (playing, set_playing) = signal(false);
    let _play = move || *set_playing.write() = true;
    let _stop = move || *set_playing.write() = false;
    let toggle_play_stop = move |_| *set_playing.write() = !(playing.get());

    let query = use_query_map();
    let params = use_params_map();
    let page = move || {
        query
            .read()
            .get("page")
            .and_then(|page| page.parse::<usize>().ok())
            .unwrap_or(1)
    };
    let story_type = move || {
        params
            .read()
            .get("stories")
            .unwrap_or_else(|| "top".to_string())
    };
    let stories = Resource::new(
        move || (page(), story_type()),
        move |(page, story_type)| async move {
            let path = format!("{}?page={}", category(&story_type), page);
            api::fetch_api::<Vec<api::Story>>(&api::story(&path)).await
        },
    );
    let (pending, set_pending) = signal(false);

    let hide_more_link = move || match &*stories.read() {
        Some(Some(stories)) => stories.len() < 28,
        _ => true
    } || pending.get();

    view! {
        <div class="container mx-auto">
            <div class="mx-auto my-4 gap-x-4 rounded-xl p-6 shadow-lg outline outline-black/5 dark:bg-slate-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10"
                class=("bg-emerald-100", move || playing.get()) class=("bg-white", move || !playing.get())>
                <button on:click=toggle_play_stop type="button" class="text-gray-700 border border-gray-700 hover:bg-gray-700 hover:text-white focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center dark:border-blue-500 dark:text-blue-500 dark:hover:text-white dark:focus:ring-blue-800 dark:hover:bg-blue-500">
                    {move || if playing.get() {
                        view! {
                            <img src="/assets/icons/player-stop-svgrepo-com.svg" alt="Stop" class="w-6 h-6"/>
                            <span class="sr-only">Stop</span>
                        }
                    } else {
                        view! {
                            <img src="/assets/icons/player-play-svgrepo-com.svg" alt="Play" class="w-6 h-6"/>
                            <span class="sr-only">Play</span>
                        }
                    }}
                </button>
            </div>
            <div class="mx-auto my-4 gap-x-4 rounded-xl bg-white p-6 shadow-lg outline outline-black/5 dark:bg-slate-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">
                <label for="volume-range" class="block font-bold text-gray-700 mb-2">Volume: {volume}%</label>
                <input id="volume-range" type="range" prop:value=move || volume.get() on:input=move |e| {
                    let val = event_target_value(&e).parse().unwrap_or(0);
                    *set_volume.write() = val;
                } class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"/>
            </div>
        </div>
    }
}