// Needed for building in Docker container
// See https://github.com/clux/muslrust?tab=readme-ov-file#diesel-and-pq-builds
// See https://github.com/sgrif/pq-sys/issues/25
#[cfg(target = "x86_64-unknown-linux-musl")]
extern crate openssl;

#[cfg(target = "x86_64-unknown-linux-musl")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "ssr")]
extern crate diesel_migrations;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::{post, get}, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use libretunes::app::*;
    use libretunes::fileserv::{file_and_error_handler, get_static_file};
    use tower_sessions::SessionManagerLayer;
    use tower_sessions_redis_store::{fred::prelude::*, RedisStore};

    use dotenv::dotenv;
    dotenv().ok();

    // Bring the database up to date
    libretunes::database::migrate();

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_config = RedisConfig::from_url(&redis_url).expect(&format!("Unable to parse Redis URL: {}", redis_url));
    let redis_pool = RedisPool::new(redis_config, None, None, None, 1).expect("Unable to create Redis pool");
    redis_pool.connect();
    redis_pool.wait_for_connect().await.expect("Unable to connect to Redis");

    let session_store = RedisStore::new(redis_pool);
    let session_layer = SessionManagerLayer::new(session_store);

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .route("/assets/*uri", get(|uri| get_static_file(uri, "")))
        .layer(session_layer)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    println!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect(&format!("Could not bind to {}", &addr));
    axum::serve(listener, app.into_make_service()).await.expect("Server failed");
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
    use leptos::*;
    use libretunes::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
