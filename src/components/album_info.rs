use leptos::prelude::*;
use crate::albumdata::AlbumData;

#[component]
pub fn AlbumInfo(albumdata: AlbumData) -> impl IntoView {
	view! {
		<div class="album-info">
			<img class="album-image" src={albumdata.image_path} alt="dashboard-tile" />
			<div class="album-body">
				<p class="album-title">{albumdata.title}</p>
				<div class="album-artists">
					{
						albumdata.artists.iter().map(|artist| {
							view! {
								<a class="album-artist" href={format!("/artist/{}", artist.id.unwrap())}>{artist.name.clone()}</a>
							}
						}).collect::<Vec<_>>()
					}
				</div>
			</div>
		</div>
	}.into_view()
}

