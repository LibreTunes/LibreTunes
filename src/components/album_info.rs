use leptos::leptos_dom::*;
use leptos::*;
use crate::albumdata::AlbumData;

#[component]
pub fn AlbumInfo(albumdata: AlbumData) -> impl IntoView {
	view! {
		<div>
			<div>
				<img src={albumdata.image_path} alt="dashboard-tile" />
			</div>
			<div>
				<p>{albumdata.title}</p>
				<div>
					{
						albumdata.artists.iter().map(|artist| {
							view! {
								<p>{artist.name.clone()}</p>
							}
						}).collect::<Vec<_>>()
					}
				</div>
			</div>
		</div>
	}.into_view()
}

