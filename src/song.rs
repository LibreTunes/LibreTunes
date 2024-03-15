use leptos::*;

#[component]
pub fn Song(song_image_path: String, song_title: String, song_artist: String) -> impl IntoView {
	view!{
		<div class="song">
			<img src={song_image_path} alt={song_title.clone()} />
			<div class="song-info">
				<h3>{song_title}</h3>
				<p>{song_artist}</p>
			</div>
		</div>
	}
}