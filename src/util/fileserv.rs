use axum::{
    body::Body,
    extract::State,
    response::IntoResponse,
    http::{Request, Response, StatusCode, Uri},
};
use axum::response::Response as AxumResponse;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use leptos::prelude::*;
use crate::app::App;
use std::str::FromStr;

pub async fn file_and_error_handler(uri: Uri, State(options): State<LeptosOptions>, req: Request<Body>) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(App);
        handler(req).await.into_response()
    }
}

pub async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await.ok() {
        Some(res) => Ok(res.into_response()),
        None => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )),
    }
}

pub enum AssetType {
    Audio,
    Image,
}

pub async fn get_asset_file(filename: String, asset_type: AssetType) -> Result<Response<Body>, (StatusCode, String)> {
    const DEFAULT_AUDIO_PATH: &str = "assets/audio";
    const DEFAULT_IMAGE_PATH: &str = "assets/images";

    let root = match asset_type {
        AssetType::Audio => std::env::var("LIBRETUNES_AUDIO_PATH").unwrap_or(DEFAULT_AUDIO_PATH.to_string()),
        AssetType::Image => std::env::var("LIBRETUNES_IMAGE_PATH").unwrap_or(DEFAULT_IMAGE_PATH.to_string()),
    };

    // Create a Uri from the filename
    // ServeDir expects a leading `/`
    let uri = Uri::from_str(format!("/{}", filename).as_str());

    match uri {
        Ok(uri) => get_static_file(uri, root.as_str()).await,
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Attempted to serve an invalid file".to_string(),
        )),
    }
}
