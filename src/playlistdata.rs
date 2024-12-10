use crate::components::dashboard_tile::DashboardTile;
use crate::models::User;
use serde::{Serialize, Deserialize};

use chrono::NaiveDateTime;

/// Holds information about a playlist
///
/// Intended to be used in the front-end
#[derive(Serialize, Deserialize, Clone)]
pub struct PlaylistData {
    /// Playlist id
    pub id: i32,
    /// Playlist title
    pub title: String,
    /// Playlist owner
    pub owner: User,
    /// Playlist creation time
    pub created_at: NaiveDateTime,
    /// Playlist update time3
    pub updated_at: NaiveDateTime,
    /// Path to playlist image, relative to the root of the web server.
    /// For example, `"/assets/images/Playlist.jpg"`
    pub image_path: String,
}

impl DashboardTile for PlaylistData {
    fn image_path(&self) -> String {
        self.image_path.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn link(&self) -> String {
        format!("/playlist/{}", self.id)
    }

    fn description(&self) -> Option<String> {
        Some(format!("Playlist by {}", self.owner.username))
    }
}