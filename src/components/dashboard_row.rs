use leptos::html::Ul;
use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn, use_scroll, UseScrollReturn};
use crate::components::dashboard_tile::DashboardTile;
use leptos_icons::*;

/// A row of dashboard tiles, with a title
pub struct DashboardRow {
	pub title: String,
	pub tiles: Vec<Box<dyn DashboardTile>>,
}

impl DashboardRow {
	pub fn new(title: String, tiles: Vec<Box<dyn DashboardTile>>) -> Self {
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
			if let Some(scroll_element) = list_ref.get_untracked() {
				let client_width = scroll_element.client_width() as f64;
				let current_pos = scroll_element.scroll_left() as f64;
				let desired_pos = current_pos - client_width;

				if let Some(first_tile) = scroll_element.first_element_child() {
					let tile_width = first_tile.client_width() as f64;
					let scroll_pos = desired_pos + (tile_width - (desired_pos % tile_width));
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
			if let Some(scroll_element) = list_ref.get_untracked() {
				let client_width = scroll_element.client_width() as f64;
				let current_pos = scroll_element.scroll_left() as f64;
				let desired_pos = current_pos + client_width;
				
				if let Some(first_tile) = scroll_element.first_element_child() {
					let tile_width = first_tile.client_width() as f64;
					let scroll_pos = desired_pos - (desired_pos % tile_width);
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

		let UseElementSizeReturn { width: scroll_element_width, .. } = use_element_size(list_ref);
		let UseScrollReturn { x: scroll_x, .. } = use_scroll(list_ref);

		let scroll_right_hidden = Signal::derive(move || {
			if let Some(scroll_element) = list_ref.get() {
				if scroll_element.scroll_width() as f64 - scroll_element_width.get() <= scroll_x.get() {
					"visibility: hidden"
				} else {
					""
				}
			} else {
				""
			}
		});

		let scroll_left_hidden = Signal::derive(move || {
			if scroll_x.get() <= 0.0 {
				"visibility: hidden"
			} else {
				""
			}
		});

		view! {
			<div class="dashboard-tile-row">
				<div class="dashboard-tile-row-title-row">
					<h2>{self.title}</h2>
					<div class="dashboard-tile-row-scroll-btn">
						<button on:click=scroll_left tabindex=-1 style=scroll_left_hidden>
							<Icon icon={icondata::FiChevronLeft} {..} class="dashboard-tile-row-scroll" />
						</button>
						<button on:click=scroll_right tabindex=-1 style=scroll_right_hidden>
							<Icon icon={icondata::FiChevronRight} {..} class="dashboard-tile-row-scroll" />
						</button>
					</div>
				</div>
				<ul _ref={list_ref}>
				{self.tiles.into_iter().map(|tile_info| {
					view! {
						<li>
							{ tile_info.into_view() }
						</li>
					}
				}).collect::<Vec<_>>()}
				</ul>
			</div>
		}.into_view()
	}
}
