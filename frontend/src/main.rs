#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use frontend::app::*;
    use tokio_util::sync::CancellationToken;
    use std::{sync::Arc, thread::sleep, time::Duration};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    let cancellation_token = Arc::new(CancellationToken::new());
    let shutdown_token = cancellation_token.clone();
    let audio_token = cancellation_token.clone();

    tokio::spawn(async move {
        println!("Waiting for Ctrl+C");
        tokio::signal::ctrl_c().await.unwrap();
        println!("Shutting down...");
        shutdown_token.cancel();
    });

    tokio::spawn(async move {
        println!("Audio processing started...");
        while !audio_token.is_cancelled() {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("Audio process tick");
        }
        println!("Audio process exiting...");
    });

    // let stop_handle = web::Data::new(StopHandle::default());

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Logger::default())
    })
    .bind(&addr)?
    .run()
    .await?;

    sleep(Duration::from_secs(3));

    // stop_handle.register(http_server.handle());

    // let server_handle = http_server.handle();

    // // Spawn a task to stop the server when the cancellation token is triggered
    // let cancel_token = cancellation_token.clone();
    // tokio::spawn(async move {
    //     cancel_token.cancelled().await;
    //     log::info!("Cancellation token triggered, stopping server...");
    //     server_handle.stop(true).await;
    // });

    println!("application successfully shut down gracefully");

    Ok(())
}

// #[cfg(feature = "ssr")]
// #[derive(Default)]
// pub(crate) struct StopHandle {
//     inner: Mutex<Option<ServerHandle>>,
// }

// #[cfg(feature = "ssr")]
// impl StopHandle {
//     /// Sets the server handle to stop.
//     pub(crate) fn register(&self, handle: ServerHandle) {
//         log::debug!("Registering stop handle");
//         *self.inner.lock() = Some(handle);
//     }

//     /// Sends stop signal through contained server handle.
//     pub(crate) fn stop(&self, graceful: bool) {
//         log::debug!("Stopping server");
//         #[allow(clippy::let_underscore_future)]
//         let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
//     }
// }

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use frontend::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
