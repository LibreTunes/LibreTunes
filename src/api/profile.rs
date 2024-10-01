use leptos::*;
use server_fn::{codec::{MultipartData, MultipartFormData}, error::NoCustomError};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::auth::get_user;
	}
}

/// Handle a user uploading a profile picture. Converts the image to webp and saves it to the server.
#[server(input = MultipartFormData, endpoint = "/profile/upload_picture")]
pub async fn upload_picture(data: MultipartData) -> Result<(), ServerFnError> {
	// Safe to unwrap - "On the server side, this always returns Some(_). On the client side, always returns None."
	let mut data = data.into_inner().unwrap();

	let field = data.next_field().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting field: {}", e)))?
		.ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("No field found".to_string()))?;

	if field.name() != Some("picture") {
		return Err(ServerFnError::ServerError("Field name is not 'picture'".to_string()));
	}

	// Get user id from session
	let user = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?;

	let user_id = user.id.ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("User has no id".to_string()))?;

	// Read the image, and convert it to webp
	use image_convert::{to_webp, WEBPConfig, ImageResource};

	let bytes = field.bytes().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting field bytes: {}", e)))?;

	let reader = std::io::Cursor::new(bytes);
	let image_source = ImageResource::from_reader(reader)
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error creating image resource: {}", e)))?;

	let profile_picture_path = format!("assets/images/profile/{}.webp", user_id);
	let mut image_target = ImageResource::from_path(&profile_picture_path);
	to_webp(&mut image_target, &image_source, &WEBPConfig::new())
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error converting image to webp: {}", e)))?;

	Ok(())
}
