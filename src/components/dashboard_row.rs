use leptos::html::Ul;
use leptos::leptos_dom::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::dashboard_tile::DashboardTile;
use leptos_icons::*;

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
		let list_ref = create_node_ref::<Ul>();

		// Scroll functions attempt to align the left edge of the scroll area with the left edge of a tile
		// This is done by scrolling to the nearest multiple of the tile width, plus some for padding

		let scroll_left = move |_| {
			if let Some(scroll_element) = list_ref.get() {
				let client_width = scroll_element.client_width() as f64;
				let current_pos = scroll_element.scroll_left() as f64;
				let desired_pos = current_pos - client_width;

				if let Some(first_tile) = scroll_element.first_element_child() {
					let tile_width = first_tile.client_width() as f64;
					let scroll_pos = desired_pos + (tile_width - (desired_pos % tile_width)) + 15.0;
					scroll_element.scroll_to_with_x_and_y(scroll_pos, 0.0);
				} else {
					warn!("Could not get first tile to scroll left");
					// Fall back to scrolling by the client width if we can't get the tile width
					scroll_element.scroll_to_with_x_and_y(desired_pos, 0.0);
				}
			} else {
				warn!("Could not get scroll element to scroll left");
			}
		};

		let scroll_right = move |_| {
			if let Some(scroll_element) = list_ref.get() {
				let client_width = scroll_element.client_width() as f64;
				let current_pos = scroll_element.scroll_left() as f64;
				let desired_pos = current_pos + client_width;
				
				if let Some(first_tile) = scroll_element.first_element_child() {
					let tile_width = first_tile.client_width() as f64;
					let scroll_pos = desired_pos - (desired_pos % tile_width) + 15.0;
					scroll_element.scroll_to_with_x_and_y(scroll_pos, 0.0);
				} else {
					warn!("Could not get first tile to scroll right");
					// Fall back to scrolling by the client width if we can't get the tile width
					scroll_element.scroll_to_with_x_and_y(desired_pos, 0.0);
				}
			} else {
				warn!("Could not get scroll element to scroll right");
			}
		};

		view! {
			<div class="dashboard-tile-row">
				<div class="dashboard-tile-row-title-row">
					<h2>{self.title}</h2>
					<div class="dashboard-tile-row-scroll-btn">
						<button on:click=scroll_left tabindex=-1>
							<Icon class="dashboard-tile-row-scroll" icon=icondata::FiChevronLeft />
						</button>
						<button on:click=scroll_right tabindex=-1>
							<Icon class="dashboard-tile-row-scroll" icon=icondata::FiChevronRight />
						</button>
					</div>
				</div>
				<ul _ref={list_ref}>
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
