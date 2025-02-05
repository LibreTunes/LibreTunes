// These "models" are used to represent the data in the database
// Diesel uses these models to generate the SQL queries that are used to interact with the database.
// These types are also used for API endpoints, for consistency. Because the file must be compiled
// for both the server and the client, we use the `cfg_attr` attribute to conditionally add
// diesel-specific attributes to the models when compiling for the serverub mod user;

pub mod album;
pub mod artist;
pub mod history_entry;
pub mod playlist;
pub mod song;
pub mod user;

pub use album::Album;
pub use artist::Artist;
pub use history_entry::HistoryEntry;
pub use playlist::Playlist;
pub use song::Song;
pub use user::User;
