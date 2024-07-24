use crate::database;
use crate::models::{Album, Artist, Song};

use time::Date;

/// Holds information about a song
/// 
/// Intended to be used in the front-end, as it includes artist and album objects, rather than just their ids.
pub struct SongData {
	/// Song id
	pub id: i32,
	/// Song name
	pub title: String,
	/// Song artists
	pub artists: Vec<Artist>,
	/// Song album
	pub album: Option<Album>,
	/// The track number of the song on the album
	pub track: Option<i32>,
	/// The duration of the song in seconds
	pub duration: i32,
	/// The song's release date
	pub release_date: Option<Date>,
	/// Path to song file, relative to the root of the web server.
	/// For example, `"/assets/audio/Song.mp3"`
	pub song_path: String,
	/// Path to song image, relative to the root of the web server.
	/// For example, `"/assets/images/Song.jpg"`
	pub image_path: String,
}
