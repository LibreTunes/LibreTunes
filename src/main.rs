// Needed for building in Docker container
// See https://github.com/clux/muslrust?tab=readme-ov-file#diesel-and-pq-builds
// See https://github.com/sgrif/pq-sys/issues/25
#[cfg(target_env = "musl")]
extern crate openssl;

#[cfg(target_env = "musl")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "ssr")]
extern crate diesel_migrations;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router, extract::Path};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use libretunes::app::*;
    use libretunes::fileserv::{file_and_error_handler, get_asset_file, get_static_file, AssetType};
    use axum_login::tower_sessions::SessionManagerLayer;
    use tower_sessions_redis_store::{fred::prelude::*, RedisStore};
    use axum_login::AuthManagerLayerBuilder;
    use libretunes::auth_backend::AuthBackend;
    use log::*;

    flexi_logger::Logger::try_with_env_or_str("debug").unwrap().format(flexi_logger::opt_format).start().unwrap();

    info!("\n{}", include_str!("../ascii_art.txt"));
    info!("Starting Leptos server...");

    use dotenv::dotenv;
    dotenv().ok();

    debug!("Running database migrations...");

    // Bring the database up to date
    libretunes::database::migrate();

    debug!("Connecting to Redis...");

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_config = RedisConfig::from_url(&redis_url).expect(&format!("Unable to parse Redis URL: {}", redis_url));
    let redis_pool = RedisPool::new(redis_config, None, None, None, 1).expect("Unable to create Redis pool");
    redis_pool.connect();
    redis_pool.wait_for_connect().await.expect("Unable to connect to Redis");

    let session_store = RedisStore::new(redis_pool);
    let session_layer = SessionManagerLayer::new(session_store);

    let auth_backend = AuthBackend;
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .route("/assets/audio/:song", get(|Path(song) : Path<String>| get_asset_file(song, AssetType::Audio)))
        .route("/assets/images/:image", get(|Path(image) : Path<String>| get_asset_file(image, AssetType::Image)))
        .route("/assets/*uri", get(|uri| get_static_file(uri, "")))
        .layer(auth_layer)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect(&format!("Could not bind to {}", &addr));

    info!("Listening on http://{}", &addr);

    axum::serve(listener, app.into_make_service()).await.expect("Server failed");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
