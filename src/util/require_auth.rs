use axum::extract::Request;
use axum::response::Response;
use axum::body::Body;
use axum::middleware::Next;
use axum_login::AuthSession;
use http::StatusCode;

use crate::util::auth_backend::AuthBackend;

use axum::extract::FromRequestParts;

// Things in pkg/ are allowed automatically. This includes the CSS/JS/WASM files
const ALLOWED_PATHS: [&str; 5] = ["/login", "/signup", "/api/login", "/api/signup", "/favicon.ico"];

/**
 * Middleware to require authentication for all paths except those in `ALLOWED_PATHS`
 * 
 * If a user is not authenticated, they will be redirected to the login page
 */
pub async fn require_auth_middleware(req: Request, next: Next) -> Result<Response<Body>, (StatusCode, &'static str)> {
	let path = req.uri().path();

	if !ALLOWED_PATHS.iter().any(|&x| x == path) {
		let (mut parts, body) = req.into_parts();

		let auth_session = AuthSession::<AuthBackend>::from_request_parts(&mut parts, &())
			.await?;

		if auth_session.user.is_none() {
			let response = Response::builder()
				.status(StatusCode::TEMPORARY_REDIRECT)
				.header("Location", "/login")
				.body(Body::empty())
				.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response"))?;

			return Ok(response);
		}

		let req = Request::from_parts(parts, body);
		let response = next.run(req).await;
		Ok(response)
	} else {
		let response = next.run(req).await;
		Ok(response)
	}
}
