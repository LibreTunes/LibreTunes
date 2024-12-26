use leptos::*;
use leptos_router::use_params_map;
use leptos_icons::*;
use server_fn::error::NoCustomError;

use crate::components::dashboard_row::DashboardRow;
use crate::components::song_list::*;
use crate::components::loading::*;
use crate::components::error::*;

use crate::api::profile::*;

use crate::models::User;
use crate::users::get_user_by_id;
use crate::util::state::GlobalState;

/// Duration in seconds backwards from now to aggregate history data for
const HISTORY_SECS: i64 = 60 * 60 * 24 * 30;
const HISTORY_MESSAGE: &str = "Last Month";

/// How many top songs to show
const TOP_SONGS_COUNT: i64 = 10;
/// How many recent songs to show
const RECENT_SONGS_COUNT: i64 = 5;
/// How many recent artists to show
const TOP_ARTISTS_COUNT: i64 = 10;

/// Profile page
/// Shows the current user's profile if no id is specified, or a user's profile if an id is specified in the path
#[component]
pub fn Profile() -> impl IntoView {
	let params = use_params_map();

	view! {
		<div class="profile-container home-component">
			{move || params.with(|params| {
				match params.get("id").map(|id| id.parse::<i32>()) {
					None => {
						// No id specified, show the current user's profile
						view! { <OwnProfile /> }.into_view()
					},
					Some(Ok(id)) => {
						// Id specified, get the user and show their profile
						view! { <UserIdProfile id /> }.into_view()
					},
					Some(Err(e)) => {
						// Invalid id, return an error
						view! {
							<Error<String>
								title="Invalid User ID"
								error=e.to_string()
							/>
						}.into_view()
					}
				}
			})}
		</div>
	}
}

/// Show the logged in user's profile
#[component]
fn OwnProfile() -> impl IntoView {
	view! {
		<Transition
			fallback=move || view! { <LoadingPage /> }
		>
			{move || GlobalState::logged_in_user().get().map(|user| {
				match user {
					Some(user) => {
						let user_id = user.id.unwrap();
						view! {
								<UserProfile user />
								<TopSongs user_id={user_id} />
								<RecentSongs user_id={user_id} />
								<TopArtists user_id={user_id} />
						}.into_view()
					},
					None => view! {
						<Error<String>
							title="Not Logged In"
							message="You must be logged in to view your profile"
						/>
					}.into_view(),
				}
			})}
		</Transition>
	}
}

/// Show a user's profile by ID
#[component]
fn UserIdProfile(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
	let user_info = create_resource(move || id.get(), move |id| {
		get_user_by_id(id)
	});

	// Show the details if the user is found
	let show_details = create_rw_signal(false);

	view!{ 
		<Transition
			fallback=move || view! { <LoadingPage /> }
		>
			{move || user_info.get().map(|user| {
				match user {
					Ok(Some(user)) => {
						show_details.set(true);

						view! { <UserProfile user /> }.into_view()
					},
					Ok(None) => {
						show_details.set(false);

						view! {
							<Error<String>
								title="User Not Found"
								message=format!("User with ID {} not found", id.get())
							/>
						}.into_view()
					},
					Err(error) => {
						show_details.set(false);

						view! {
							<ServerError<NoCustomError>
								title="Error Getting User"
								error
							/>
						}.into_view()
					}
				}
			})}
		</Transition>
		<div hidden={move || !show_details.get()}>
			<TopSongs user_id={id} />
			<RecentSongs user_id={id} />
			<TopArtists user_id={id} />
		</div>
	}
}

/// Show a profile for a User object
#[component]
fn UserProfile(user: User) -> impl IntoView {
	let user_id = user.id.unwrap();
	let profile_image_path = format!("/assets/images/profile/{}.webp", user_id);

	view! {
		<div class="profile-header">
			<object class="profile-image" data={profile_image_path.clone()} type="image/webp">
				<Icon class="profile-image" icon=icondata::CgProfile width="75" height="75"/>
			</object>
			<h1>{user.username}</h1>
		</div>
		<div class="profile-details">
			<p>
				{user.email}
				{
					user.created_at.map(|created_at| {
						format!(" • Joined {}", created_at.format("%B %Y"))
					})
				}
				{
					if user.admin {
						" • Admin"
					} else {
						""
					}
				}
			</p>
		</div>
	}
}

/// Show a list of top songs for a user
#[component]
fn TopSongs(#[prop(into)] user_id: MaybeSignal<i32>) -> impl IntoView {
	let top_songs = create_resource(move || user_id.get(), |user_id| async move {
		use chrono::{Local, Duration};
		let now = Local::now();
		let start = now - Duration::seconds(HISTORY_SECS);
		let top_songs = top_songs(user_id, start.naive_utc(), now.naive_utc(), Some(TOP_SONGS_COUNT)).await;

		top_songs.map(|top_songs| {
			top_songs.into_iter().map(|(plays, song)| {
				let plays = if plays == 1 {
					format!("{} Play", plays)
				} else {
					format!("{} Plays", plays)
				};

				(song, plays)
			}).collect::<Vec<_>>()
		})
	});

	view! {
		<h2>{format!("Top Songs {}", HISTORY_MESSAGE)}</h2>
		<Transition
			fallback=move || view! { <Loading /> }
		>
			<ErrorBoundary
				fallback=|errors| view! {
					{move || errors.get()
						.into_iter()
						.map(|(_, e)| view! { <p>{e.to_string()}</p>})
						.collect_view()
					}
				}
			>
				{move ||
					top_songs.get().map(|top_songs| {
						top_songs.map(|top_songs| {
							view! {
								<SongListExtra songs=top_songs />
							}
						})
					})
				}
			</ErrorBoundary>
		</Transition>
	}
}

/// Show a list of recently played songs for a user
#[component]
fn RecentSongs(#[prop(into)] user_id: MaybeSignal<i32>) -> impl IntoView {
	let recent_songs = create_resource(move || user_id.get(), |user_id| async move {
		let recent_songs = recent_songs(user_id, Some(RECENT_SONGS_COUNT)).await;

		recent_songs.map(|recent_songs| {
			recent_songs.into_iter().map(|(_date, song)| {
				song
			}).collect::<Vec<_>>()
		})
	});

	view! {
		<h2>"Recently Played"</h2>
		<Transition
			fallback=move || view! { <Loading /> }
		>
			<ErrorBoundary
				fallback=|errors| view! {
					{move || errors.get()
						.into_iter()
						.map(|(_, e)| view! { <p>{e.to_string()}</p>})
						.collect_view()
					}
				}
			>
				{move ||
					recent_songs.get().map(|recent_songs| {
						recent_songs.map(|recent_songs| {
							view! {
								<SongList songs=recent_songs />
							}
						})
					})
				}
			</ErrorBoundary>	
		</Transition>
	}
}

/// Show a list of top artists for a user
#[component]
fn TopArtists(#[prop(into)] user_id: MaybeSignal<i32>) -> impl IntoView {
	let top_artists = create_resource(move || user_id.get(), |user_id| async move {
		use chrono::{Local, Duration};

		let now = Local::now();
		let start = now - Duration::seconds(HISTORY_SECS);
		let top_artists = top_artists(user_id, start.naive_utc(), now.naive_utc(), Some(TOP_ARTISTS_COUNT)).await;

		top_artists.map(|top_artists| {
			top_artists.into_iter().map(|(_plays, artist)| {
				artist
			}).collect::<Vec<_>>()
		})
	});

	view! {
		<Transition
			fallback=move || view! {
				<h2>{format!("Top Artists {}", HISTORY_MESSAGE)}</h2>
				<Loading />
			}
		>
			<ErrorBoundary
				fallback=|errors| view! {
					<h2>{format!("Top Artists {}", HISTORY_MESSAGE)}</h2>
					{move || errors.get()
						.into_iter()
						.map(|(_, e)| view! { <p>{e.to_string()}</p>})
						.collect_view()
					}
				}
			>
				{move ||
					top_artists.get().map(|top_artists| {
						top_artists.map(|top_artists| {
							let tiles = top_artists.into_iter().map(|artist| {
								artist.into()
							}).collect::<Vec<_>>();

							view! {
								<DashboardRow title=format!("Top Artists {}", HISTORY_MESSAGE) tiles />
							}
						})
					})
				}
			</ErrorBoundary>
		</Transition>
	}
}
