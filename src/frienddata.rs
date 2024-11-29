use serde::{Serialize, Deserialize};

use chrono::NaiveDate;

/// Holds information about an album
/// 
/// Intended to be used in the front-end

#[derive(Serialize, Deserialize, Clone)]
pub struct FriendData {
	/// Username
	pub username: String,
	/// Date which the friend was added
	pub created_at: NaiveDate,
	/// User's id to be used to locate their profile image
	pub user_id: i32
}
