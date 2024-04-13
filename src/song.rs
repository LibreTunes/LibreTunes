use leptos::*;
use leptos::logging::*;
use crate::api::songs::get_artists;

#[component]
pub fn Song(song_id_arg: Option<i32>, song_image_path: String, song_title: String) -> impl IntoView {

	let song_id = Signal::derive(move || song_id_arg);

	let song_artists_resource = create_resource(song_id, move |song_id| async move {
		let artists_vec = get_artists(song_id).await.unwrap_or_else(|_| {
			warn!("Error when searching for artists for song");
			Vec::new()
		});
		// convert the vec of artists to a string of artists separated by commas
		let artists_string = artists_vec.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", ");
		artists_string
	});

	view!{
		<div class="queue-song">
			<img src={song_image_path} alt={song_title.clone()} />
			<div class="queue-song-info">
				<h3>{song_title}</h3>
				<Suspense
					fallback=move || view! {<p class="fallback-artists">""</p>}
				>
				{move || {
					song_artists_resource.get().map(|artists_string| {
						if artists_string.is_empty() {
							view! {<p class="fallback-artists">""</p>}
						}
						else {
							view! {<p class="artists">{artists_string}</p>}
						}
					})
				}}
				</Suspense>
			</div>
		</div>
	}
}