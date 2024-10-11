use leptos::*;
use leptos_icons::*;

use crate::songdata::SongData;
use crate::models::{Album, Artist};

const LIKE_DISLIKE_BTN_SIZE: &str = "2em";

#[component]
pub fn SongList(songs: MaybeSignal<Vec<SongData>>) -> impl IntoView {
	view! {
		<table class="song-list">
			{
				songs.with(|songs| {
					let mut first_song = true;

					songs.iter().map(|song| {
						let playing = first_song.into();
						first_song = false;

						view! {
							<SongListItem song={song.clone()} song_playing=playing />
						}
					}).collect::<Vec<_>>()
				})
			}
		</table>
	}
}

#[component]
pub fn SongListItem(song: SongData, song_playing: MaybeSignal<bool>) -> impl IntoView {
	let liked = create_rw_signal(song.like_dislike.map(|(liked, _)| liked).unwrap_or(false));
	let disliked = create_rw_signal(song.like_dislike.map(|(_, disliked)| disliked).unwrap_or(false));
	
	view! {
		<tr class="song-list-item">
			<td class="song-image"><SongImage image_path=song.image_path song_playing /></td>
			<td class="song-title"><p>{song.title}</p></td>
			<td class="song-list-spacer"></td>
			<td class="song-artists"><SongArtists artists=song.artists /></td>
			<td class="song-list-spacer"></td>
			<td class="song-album"><SongAlbum album=song.album /></td>
			<td class="song-list-spacer-big"></td>
			<td class="song-like-dislike"><SongLikeDislike liked disliked/></td>
			<td>{format!("{}:{:02}", song.duration / 60, song.duration % 60)}</td>
		</tr>
	}
}

/// Display the song's image, with an overlay if the song is playing
/// When the song list item is hovered, the overlay will show the play button
#[component]
fn SongImage(image_path: String, song_playing: MaybeSignal<bool>) -> impl IntoView {
	view! {
		<img class="song-image" src={image_path}/>
		{if song_playing.get() {
			view! { <Icon class="song-image-overlay song-playing-overlay" icon=icondata::BsPauseFill /> }.into_view()
		} else {
			view! { <Icon class="song-image-overlay hide-until-hover" icon=icondata::BsPlayFill /> }.into_view()
		}}
	}
}

/// Displays a song's artists, with links to their artist pages
#[component]
fn SongArtists(artists: Vec<Artist>) -> impl IntoView {
	let num_artists = artists.len() as isize;

	artists.iter().enumerate().map(|(i, artist)| {
		let i = i as isize;
		
		view! {
			{
				if let Some(id) = artist.id {
					view! { <a href={format!("/artist/{}", id)}>{artist.name.clone()}</a> }.into_view()
				} else {
					view! { <span>{artist.name.clone()}</span> }.into_view()
				}
			}
			{if i < num_artists - 2 { ", " } else if i == num_artists - 2 { " & " } else { "" }}
		}
	}).collect::<Vec<_>>()
}

/// Display a song's album, with a link to the album page
#[component]
fn SongAlbum(album: Option<Album>) -> impl IntoView {
	album.as_ref().map(|album| {
		view! {
			<span>
				{
					if let Some(id) = album.id {
						view! { <a href={format!("/album/{}", id)}>{album.title.clone()}</a> }.into_view()
					} else {
						view! { <span>{album.title.clone()}</span> }.into_view()
					}
				}
			</span>
		}
	})
}

/// Display like and dislike buttons for a song, and indicate if the song is liked or disliked
#[component]
fn SongLikeDislike(liked: RwSignal<bool>, disliked: RwSignal<bool>) -> impl IntoView {
	let like_icon = Signal::derive(move || {
		if liked.get() {
			icondata::TbThumbUpFilled
		} else {
			icondata::TbThumbUp
		}
	});

	let dislike_icon = Signal::derive(move || {
		if disliked.get() {
			icondata::TbThumbDownFilled
		} else {
			icondata::TbThumbDown
		}
	});

	let like_class = MaybeProp::derive(move || {
		if liked.get() {
			Some(TextProp::from("controlbtn"))
		} else {
			Some(TextProp::from("controlbtn hide-until-hover"))
		}
	});

	let dislike_class = MaybeProp::derive(move || {
		if disliked.get() {
			Some(TextProp::from("controlbtn hmirror"))
		} else {
			Some(TextProp::from("controlbtn hmirror hide-until-hover"))
		}
	});

	let toggle_like = move |_| {
		liked.set(!liked.get_untracked());
		disliked.set(disliked.get_untracked() && !liked.get_untracked());
	};

	let toggle_dislike = move |_| {
		disliked.set(!disliked.get_untracked());
		liked.set(liked.get_untracked() && !disliked.get_untracked());
	};

	view! {
		<button on:click=toggle_dislike>
			<Icon class=dislike_class width=LIKE_DISLIKE_BTN_SIZE height=LIKE_DISLIKE_BTN_SIZE icon=dislike_icon />
		</button>
		<button on:click=toggle_like>
			<Icon class=like_class width=LIKE_DISLIKE_BTN_SIZE height=LIKE_DISLIKE_BTN_SIZE icon=like_icon />
		</button>
	}
}
