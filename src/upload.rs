use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use multer::Field;
		use crate::database::get_db_conn;
		use diesel::prelude::*;
		use log::*;
		use server_fn::error::NoCustomError;
		use chrono::NaiveDate;
	}
}

/// Extract the text from a multipart field
#[cfg(feature = "ssr")]
async fn extract_field(field: Field<'static>) -> Result<String, ServerFnError> {
	let field = match field.text().await {
		Ok(field) => field,
		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error reading field: {}", e)))?,
	};

	Ok(field)
}

/// Validate the artist ids in a multipart field
/// Expects a field with a comma-separated list of artist ids, and ensures each is a valid artist id in the database
#[cfg(feature = "ssr")]
async fn validate_artist_ids(artist_ids: Field<'static>) -> Result<Vec<i32>, ServerFnError> {
    use crate::models::Artist;
	use diesel::result::Error::NotFound;

	// Extract the artist id from the field
	match artist_ids.text().await {
		Ok(artist_ids) => {
			let artist_ids = artist_ids.trim_end_matches(',').split(',');

			artist_ids.filter(|artist_id| !artist_id.is_empty()).map(|artist_id| {
				// Parse the artist id as an integer
				if let Ok(artist_id) = artist_id.parse::<i32>() {
					// Check if the artist exists
					let db_con = &mut get_db_conn();
					let artist = crate::schema::artists::dsl::artists.find(artist_id).first::<Artist>(db_con);

					match artist {
						Ok(_) => Ok(artist_id),
						Err(NotFound) => Err(ServerFnError::<NoCustomError>::
							ServerError("Artist does not exist".to_string())),
						Err(e) => Err(ServerFnError::<NoCustomError>::
							ServerError(format!("Error finding artist id: {}", e))),
					}
				} else {
					Err(ServerFnError::<NoCustomError>::ServerError("Error parsing artist id".to_string()))
				}
			}).collect()
		},

		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error reading artist id: {}", e))),
	}
}

/// Validate the album id in a multipart field
/// Expects a field with an album id, and ensures it is a valid album id in the database
#[cfg(feature = "ssr")]
async fn validate_album_id(album_id: Field<'static>) -> Result<Option<i32>, ServerFnError> {
	use crate::models::Album;
	use diesel::result::Error::NotFound;

	// Extract the album id from the field
	match album_id.text().await {
		Ok(album_id) => {
			if album_id.is_empty() {
				return Ok(None);
			}

			// Parse the album id as an integer
			if let Ok(album_id) = album_id.parse::<i32>() {
				// Check if the album exists
				let db_con = &mut get_db_conn();
				let album = crate::schema::albums::dsl::albums.find(album_id).first::<Album>(db_con);

				match album {
					Ok(_) => Ok(Some(album_id)),
					Err(NotFound) => Err(ServerFnError::<NoCustomError>::
						ServerError("Album does not exist".to_string())),
					Err(e) => Err(ServerFnError::<NoCustomError>::
						ServerError(format!("Error finding album id: {}", e))),
				}
			} else {
				Err(ServerFnError::<NoCustomError>::ServerError("Error parsing album id".to_string()))
			}
		},
		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error reading album id: {}", e))),
	}
}

/// Validate the track number in a multipart field
/// Expects a field with a track number, and ensures it is a valid track number (non-negative integer)
#[cfg(feature = "ssr")]
async fn validate_track_number(track_number: Field<'static>) -> Result<Option<i32>, ServerFnError> {
	match track_number.text().await {
		Ok(track_number) => {
			if track_number.is_empty() {
				return Ok(None);
			}

			if let Ok(track_number) = track_number.parse::<i32>() {
				if track_number < 0 {
					return Err(ServerFnError::<NoCustomError>::
						ServerError("Track number must be positive or 0".to_string()));
				} else {
					Ok(Some(track_number))
				}
			} else {
				return Err(ServerFnError::<NoCustomError>::ServerError("Error parsing track number".to_string()));
			}
		},
		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error reading track number: {}", e)))?,
	}
}

/// Validate the release date in a multipart field
/// Expects a field with a release date, and ensures it is a valid date in the format [year]-[month]-[day]
#[cfg(feature = "ssr")]
async fn validate_release_date(release_date: Field<'static>) -> Result<Option<NaiveDate>, ServerFnError> {
	match release_date.text().await {
		Ok(release_date) => {
			if release_date.trim().is_empty() {
				return Ok(None);
			}

			let release_date = NaiveDate::parse_from_str(&release_date.trim(), "%Y-%m-%d");

			match release_date {
				Ok(release_date) => Ok(Some(release_date)),
				Err(_) => Err(ServerFnError::<NoCustomError>::ServerError("Invalid release date".to_string())),
			}
		},
		Err(e) => Err(ServerFnError::<NoCustomError>::ServerError(format!("Error reading release date: {}", e))),
	}
}

/// Handle the file upload form
#[server(input = MultipartFormData, endpoint = "/upload")]
pub async fn upload(data: MultipartData) -> Result<(), ServerFnError> {
	// Safe to unwrap - "On the server side, this always returns Some(_). On the client side, always returns None."
	let mut data = data.into_inner().unwrap();

	let mut title = None;
	let mut artist_ids = None;
	let mut album_id = None;
	let mut track = None;
	let mut release_date = None;
	let mut file_name = None;
	let mut duration = None;

	// Fetch the fields from the form data
	while let Ok(Some(mut field)) = data.next_field().await {
		let name = field.name().unwrap_or_default().to_string();

		match name.as_str() {
			"title" => { title = Some(extract_field(field).await?); },
			"artist_ids" => { artist_ids = Some(validate_artist_ids(field).await?); },
			"album_id" => { album_id = Some(validate_album_id(field).await?); },
			"track_number" => { track = Some(validate_track_number(field).await?); },
			"release_date" => { release_date = Some(validate_release_date(field).await?); },
			"file" => {
				use symphonia::core::codecs::CODEC_TYPE_MP3;
				use crate::util::audio::extract_metadata;
				use std::fs::OpenOptions;
				use std::io::{Seek, Write};

				// Some logging is done here where there is high potential for bugs / failures,
				// or behavior that we may wish to change in the future

				// Create file name
				let title = title.clone().ok_or(ServerFnError::<NoCustomError>::
					ServerError("Title field required and must precede file field".to_string()))?;

				let clean_title = title.replace(" ", "_").replace("/", "_");
				let date_str = chrono::Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string();
				let upload_path = format!("assets/audio/upload-{}_{}.mp3", date_str, clean_title);
				file_name = Some(format!("upload-{}_{}.mp3", date_str, clean_title));

				debug!("Saving uploaded file {}", upload_path);

				// Save file to disk
				// Use these open options to create the file, write to it, then read from it
				let mut file = OpenOptions::new()
					.read(true)
					.write(true)
					.create(true)
					.open(upload_path.clone())?;

				while let Some(chunk) = field.chunk().await? {
					file.write(&chunk)?;
				}

				file.flush()?;

				// Rewind the file so the duration can be measured
				file.rewind()?;

				// Get the codec and duration of the file
				let (file_codec, file_duration) = extract_metadata(file)
					.map_err(|e| {
						let msg = format!("Error measuring duration of audio file {}: {}", upload_path, e);
						warn!("{}", msg);
						ServerFnError::<NoCustomError>::ServerError(msg)
					})?;

				if file_codec != CODEC_TYPE_MP3 {
					let msg = format!("Invalid uploaded audio file codec: {}", file_codec);
					warn!("{}", msg);
					return Err(ServerFnError::<NoCustomError>::ServerError(msg));
				}

				duration = Some(file_duration);
			},
			_ => {
				warn!("Unknown file upload field: {}", name);
			}
		}
	}

	// Unwrap mandatory fields
	let title = title.ok_or(ServerFnError::<NoCustomError>::ServerError("Missing title".to_string()))?;
	let artist_ids = artist_ids.unwrap_or(vec![]);
	let file_name = file_name.ok_or(ServerFnError::<NoCustomError>::ServerError("Missing file".to_string()))?;
	let duration = duration.ok_or(ServerFnError::<NoCustomError>::ServerError("Missing duration".to_string()))?;
	let duration = i32::try_from(duration).map_err(|e| ServerFnError::<NoCustomError>::
		ServerError(format!("Error converting duration to i32: {}", e)))?;

	let album_id = album_id.unwrap_or(None);
	let track = track.unwrap_or(None);
	let release_date = release_date.unwrap_or(None);

	if album_id.is_some() != track.is_some() {
		return Err(ServerFnError::<NoCustomError>
			::ServerError("Album id and track number must both be present or both be absent".to_string()));
	}

	// Create the song
	use crate::models::Song;
	let song = Song {
		id: None,
		title,
		album_id,
		track,
		duration,
		release_date,
		storage_path: file_name,
		image_path: None,
		added_date: None, // Defaults to current date
	};

	// Save the song to the database
	let db_con = &mut get_db_conn();
	let song = song.insert_into(crate::schema::songs::table)
		.get_result::<Song>(db_con)
		.map_err(|e| {
			let msg = format!("Error saving song to database: {}", e);
			warn!("{}", msg);
			ServerFnError::<NoCustomError>::ServerError(msg)
		})?;

	// Save the song's artists to the database
	let song_id = song.id.ok_or_else(|| {
		let msg = "Error saving song to database: song id not found after insertion".to_string();
		warn!("{}", msg);
		ServerFnError::<NoCustomError>::ServerError(msg)
	})?;

	use crate::schema::song_artists;
	use diesel::ExpressionMethods;

	let artist_ids = artist_ids.into_iter().map(|artist_id| {
		(song_artists::song_id.eq(song_id), song_artists::artist_id.eq(artist_id))
	}).collect::<Vec<_>>();

	diesel::insert_into(crate::schema::song_artists::table)
		.values(&artist_ids)
		.execute(db_con)
		.map_err(|e| {
			let msg = format!("Error saving song artists to database: {}", e);
			warn!("{}", msg);
			ServerFnError::<NoCustomError>::ServerError(msg)
		})?;

	Ok(())
}
