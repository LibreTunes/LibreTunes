/// Holds information about a song
#[derive(Debug, Clone)]
pub struct SongData {
	/// Song name
	pub name: String,
	/// Song artist
	pub artist: String,
	/// Song album
	pub album: String,
	/// Path to song file, relative to the root of the web server.
	/// For example, `"/assets/audio/Song.mp3"`
	pub song_path: String,
	/// Path to song image, relative to the root of the web server.
	/// For example, `"/assets/images/Song.jpg"`
	pub image_path: String,
}
