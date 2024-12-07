use leptos::*;
use cfg_if::cfg_if;
use crate::frienddata::FriendData;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::auth::get_user;
		use server_fn::error::NoCustomError;

		use crate::database::get_db_conn;
		use diesel::prelude::*;
		use diesel::dsl::exists;
		use crate::models::*;
		use crate::schema::*;

		use chrono::prelude::*;
	}
}

/// Get a user's list of friends from the database
#[server(endpoint = "/profile/friends")]
pub async fn friends(for_user_id: i32)
	-> Result<Vec<FriendData>, ServerFnError>
{
	let mut db_con = get_db_conn();

	let friends = friendships::table
		.filter(friendships::friend_1_id.eq(for_user_id))
		.filter(friendships::friend_1_id.ne(friendships::friend_2_id))
		.inner_join(users::table.on(users::id.eq(friendships::friend_2_id)))
		.select((users::all_columns, friendships::created_at))
		.order(friendships::created_at.desc())
		.order(users::username.asc())
		.union(
			friendships::table
			.filter(friendships::friend_2_id.eq(for_user_id))
			.filter(friendships::friend_1_id.ne(friendships::friend_2_id))
			.inner_join(users::table.on(users::id.eq(friendships::friend_1_id)))
			.select((users::all_columns, friendships::created_at))
			.order(friendships::created_at.desc())
			.order(users::username.asc())
		)
		.load(&mut db_con)?;
	
	let friend_list: Vec<FriendData> = friends.into_iter().map(|(user, created_at): (User, NaiveDateTime)| {
		FriendData {
			username: user.username,
			created_at: created_at.into(),
			user_id: user.id.unwrap()
		}
	}).collect();

	Ok(friend_list)
}

/// Get a user's list of friend requests (outgoing) from the database
#[server(endpoint = "/profile/friend-requests-outgoing")]
pub async fn friend_requests_outgoing(for_user_id: i32)
	-> Result<Vec<FriendData>, ServerFnError>
{
	let mut db_con = get_db_conn();

	let friends = friend_requests::table
		.filter(friend_requests::from_id.eq(for_user_id))
		.filter(friend_requests::from_id.ne(friend_requests::to_id))
		.inner_join(users::table.on(users::id.eq(friend_requests::to_id)))
		.select((users::all_columns, friend_requests::created_at))
		.order(friend_requests::created_at.desc())
		.order(users::username.asc())
		.load(&mut db_con)?;
	
	let friend_list: Vec<FriendData> = friends.into_iter().map(|(user, created_at): (User, NaiveDateTime)| {
		FriendData {
			username: user.username,
			created_at: created_at.into(),
			user_id: user.id.unwrap()
		}
	}).collect();

	Ok(friend_list)
}

/// Get a user's list of friend requests (incoming) from the database
#[server(endpoint = "/profile/friend-requests-incoming")]
pub async fn friend_requests_incoming(for_user_id: i32)
	-> Result<Vec<FriendData>, ServerFnError>
{
	let mut db_con = get_db_conn();

	let friends = friend_requests::table
		.filter(friend_requests::to_id.eq(for_user_id))
		.filter(friend_requests::from_id.ne(friend_requests::to_id))
		.inner_join(users::table.on(users::id.eq(friend_requests::from_id)))
		.select((users::all_columns, friend_requests::created_at))
		.order(friend_requests::created_at.desc())
		.order(users::username.asc())
		.load(&mut db_con)?;
	
	let friend_list: Vec<FriendData> = friends.into_iter().map(|(user, created_at): (User, NaiveDateTime)| {
		FriendData {
			username: user.username,
			created_at: created_at.into(),
			user_id: user.id.unwrap()
		}
	}).collect();

	Ok(friend_list)
}

/// Send a friend request
#[server(endpoint = "/profile/send-friend-request")]
pub async fn send_friend_request(to_user_id: i32)
	-> Result<(), ServerFnError>
{
	let mut db_con = get_db_conn();

	// Get user id from session
	let user = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?;

	// Get current time for request
	let timestamp: NaiveDateTime = Utc::now().naive_utc();

	// Insert into database (if already exists, won't succeed due to primary key)
	diesel::insert_into(crate::schema::friend_requests::table)
		.values((friend_requests::created_at.eq(timestamp),friend_requests::from_id.eq(user.id.unwrap()),friend_requests::to_id.eq(to_user_id)))
		.execute(&mut db_con)
		.map_err(|e| {
			let msg = format!("Error saving friend request to database: {}", e);
			ServerFnError::<NoCustomError>::ServerError(msg)
		})?;
	
	Ok(())
}

/// Remove an outgoing friend request
#[server(endpoint = "/profile/friend-requests-incoming")]
pub async fn delete_friend_request(to_user_id: i32)
	-> Result<(), ServerFnError>
{
	let mut db_con = get_db_conn();

	// Get user id from session
	let user = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?;

	// Delete the friend request
	diesel::delete(friend_requests::table
		.filter(friend_requests::from_id.eq(user.id.unwrap()))
		.filter(friend_requests::to_id.eq(to_user_id))
	).execute(&mut db_con)?;

	Ok(())
}

/// Remove an existing friendship
#[server(endpoint = "/profile/delete-friend")]
pub async fn delete_friend(for_user_id: i32)
	-> Result<(), ServerFnError>
{
	let mut db_con = get_db_conn();

	// Get user id from session
	let user = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?;

	// Delete the friend request
	diesel::delete(friendships::table
		.filter(friendships::friend_1_id.eq(user.id.unwrap()))
		.filter(friendships::friend_2_id.eq(for_user_id))
	).execute(&mut db_con)?;

	diesel::delete(friendships::table
		.filter(friendships::friend_2_id.eq(user.id.unwrap()))
		.filter(friendships::friend_1_id.eq(for_user_id))
	).execute(&mut db_con)?;

	Ok(())
}

/// Accept a friend request
#[server(endpoint = "/profile/accept-friend-request")]
pub async fn accept_friend_request(to_user_id: i32)
	-> Result<(), ServerFnError>
{
	let mut db_con = get_db_conn();

	// Get user id from session
	let user = get_user().await
		.map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Error getting user: {}", e)))?;

	// Get current time for request
	let timestamp: NaiveDateTime = Utc::now().naive_utc();

	// Make sure the person has received a friend request from the other person
	let req = diesel::select(exists(
		friend_requests::table
			.filter(friend_requests::from_id.eq(user.id.unwrap()))
			.filter(friend_requests::to_id.eq(to_user_id))
	)).get_result::<bool>(&mut db_con)?;

	if req == false {
		Err(ServerFnError::<NoCustomError>::ServerError(format!("Error, the friend request does not exist!")))?;
	}

	// Delete the friend requests
	diesel::delete(friend_requests::table
		.filter(friend_requests::from_id.eq(user.id.unwrap()))
		.filter(friend_requests::to_id.eq(to_user_id))
	).execute(&mut db_con)?;

	diesel::delete(friend_requests::table
		.filter(friend_requests::to_id.eq(user.id.unwrap()))
		.filter(friend_requests::from_id.eq(to_user_id))
	).execute(&mut db_con)?;

	// Add the new friend request either direction
	diesel::insert_into(crate::schema::friendships::table)
		.values((friendships::created_at.eq(timestamp),friendships::friend_1_id.eq(user.id.unwrap()),friendships::friend_2_id.eq(to_user_id)))
		.execute(&mut db_con)
		.map_err(|e| {
			let msg = format!("Error saving friendship to database: {}", e);
			ServerFnError::<NoCustomError>::ServerError(msg)
	})?;
	
	Ok(())
}