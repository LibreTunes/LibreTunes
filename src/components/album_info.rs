use leptos::leptos_dom::*;
use leptos::*;
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
								<p class="album-artist">{artist.name.clone()}</p>
							}
						}).collect::<Vec<_>>()
					}
				</div>
			</div>
		</div>
	}.into_view()
}

