use leptos::leptos_dom::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::dashboard_tile::DashboardTile;

/// A row of dashboard tiles, with a title
#[derive(Serialize, Deserialize)]
pub struct DashboardRow {
	pub title: String,
	pub tiles: Vec<DashboardTile>,
}

impl DashboardRow {
	pub fn new(title: String, tiles: Vec<DashboardTile>) -> Self {
		Self {
			title,
			tiles,
		}
	}
}

impl IntoView for DashboardRow {
	fn into_view(self) -> View {
		view! {
			<div class="dashboard-tile-row">
				<h2>{self.title}</h2>
				<ul>
				{self.tiles.into_iter().map(|tile_info| {
					view! {
						<li>
							{ tile_info }
						</li>
					}
				}).collect::<Vec<_>>()}
				</ul>
			</div>
		}.into_view()
	}
}
