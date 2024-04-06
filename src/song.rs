use leptos::*;
use crate::api::songs::get_artists;

#[component]
pub fn Song(song_id_arg: Option<i32>, song_image_path: String, song_title: String) -> impl IntoView {

	let song_artists_resource = create_resource(|| (), move |_| async move {
		let artists_vec = get_artists(song_id_arg).await.unwrap_or(Vec::new());
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
					fallback=move || view! { <h3>Loading...</h3> }
				>
				{move || {
					song_artists_resource.get().map(|artists_string| view! {
						<h3>{artists_string}</h3>
					})
				}}
				</Suspense>
			</div>
		</div>
	}
}