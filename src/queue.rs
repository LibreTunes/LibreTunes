use crate::playstatus::PlayStatus;
use leptos::leptos_dom::*;
use leptos::*;

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
							.map(|song| view! {
								<Song song_image_path=song.image_path.clone() song_title=song.name.clone() song_artist=song.artist.clone() />
							})
							.collect::<Vec<_>>())
					}
				</ul>
			</div>
		</Show>

	}
}
