use crate::playstatus::PlayStatus;
use crate::playbar::skip_to;
use crate::playbar::get_song_time_duration;
use crate::playbar::set_playing;
use leptos::ev::MouseEvent;
use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use leptos_icons::BsIcon::*;

const RM_BTN_SIZE: &str = "2rem";

fn skip_to_next(status: RwSignal<PlayStatus>) {
	
	if let Some(duration) = get_song_time_duration(status) {
		skip_to(status, duration.1);
		set_playing(status, true);
	} else {
		error!("Unable to skip forward: Unable to get current duration");
	}
}

fn remove_song_fn(index: usize, status: RwSignal<PlayStatus>) {
	// handle the case when index is 0 (i.e. the first song in the queue is removed)
	// if the song is currently playing, skip to the next song
	if index == 0 {
		log!("Remove Song from Queue: Song is currently playing, skipping to next song and adding to history");
		skip_to_next(status);
	} else {
		log!("Remove Song from Queue: Song is not currently playing, no need to skip to next song, instead deleting song from queue and not adding to history");
		status.update(|status| {
			status.queue.remove(index);
		});
	}
}

#[component]
fn Song(song_image_path: String, song_title: String, song_artist: String) -> impl IntoView {
	view!{
		<div class="queue-song">
			<img src={song_image_path} alt={song_title.clone()} />
			<div class="queue-song-info">
				<h3>{song_title}</h3>
				<p>{song_artist}</p>
			</div>
		</div>
	}
}

#[component]
pub fn Queue(status: RwSignal<PlayStatus>) -> impl IntoView {

	let remove_song = move |index: usize| {
		remove_song_fn(index, status);
		log!("Removed song {}", index + 1);
	};

	let prevent_focus = move |e: MouseEvent| {
        e.prevent_default();
    };

	view!{
		<Show
			when=move || status.with(|status| status.queue_open)
			fallback=|| view!{""}>
			<div class="queue">
				<div class="queue-header">
					<h2>Queue</h2>
				</div>
				<ul>
					{
						status.with(|status| status.queue.iter()
							.enumerate()
							.map(|(index, song)| view! {
								<div class="queue-item">
									<Song song_image_path=song.image_path.clone() song_title=song.name.clone() song_artist=song.artist.clone() />
									<button on:click=move |_| remove_song(index) on:mousedown=prevent_focus>
									<Icon class="remove-song" width=RM_BTN_SIZE height=RM_BTN_SIZE icon=Icon::from(BsTrashFill) />
									</button>
								</div>
							})
							.collect::<Vec<_>>())
					}
				</ul>
			</div>
		</Show>

	}
}
