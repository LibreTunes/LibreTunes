use crate::models::Artist;
use crate::song::Song;
use crate::util::state::GlobalState;
use leptos::ev::MouseEvent;
use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use leptos::ev::DragEvent;

const RM_BTN_SIZE: &str = "2.5rem";

fn remove_song_fn(index: usize) {
	if index == 0 {
		log!("Error: Trying to remove currently playing song (index 0) from queue");
	} else {
		log!("Remove Song from Queue: Song is not currently playing, deleting song from queue and not adding to history");
		GlobalState::play_status().update(|status| {
			status.queue.remove(index);
		});
	}
}

#[component]
pub fn Queue() -> impl IntoView {
	let status = GlobalState::play_status();

	let remove_song = move |index: usize| {
		remove_song_fn(index);
		log!("Removed song {}", index + 1);
	};

	let prevent_focus = move |e: MouseEvent| {
        e.prevent_default();
    };

	let index_being_dragged = create_rw_signal(-1);

	let index_being_hovered = create_rw_signal(-1);

	let on_drag_start = move |_e: DragEvent, index: usize| {
		// set the index of the item being dragged
		index_being_dragged.set(index as i32);
    };
	
	let on_drop = move |e: DragEvent| {
		e.prevent_default();
		// if the index of the item being dragged is not the same as the index of the item being hovered over
		if index_being_dragged.get() != index_being_hovered.get() && index_being_dragged.get() > 0 && index_being_hovered.get() > 0 {
			// get the index of the item being dragged
			let dragged_index = index_being_dragged.get_untracked() as usize;
			// get the index of the item being hovered over
			let hovered_index = index_being_hovered.get_untracked() as usize;
			// update the queue
			status.update(|status| {
				// remove the dragged item from the list
				let dragged_item = status.queue.remove(dragged_index);
				// insert the dragged item at the index of the item being hovered over
				status.queue.insert(hovered_index, dragged_item.unwrap());
			});
			// reset the index of the item being dragged
			index_being_dragged.set(-1);
			// reset the index of the item being hovered over
			index_being_hovered.set(-1);
			log!("drag end. Moved item from index {} to index {}", dragged_index, hovered_index);
		}
		else {
			// reset the index of the item being dragged
			index_being_dragged.set(-1);
			// reset the index of the item being hovered over
			index_being_hovered.set(-1);
		}
	};

	let on_drag_enter = move |_e: DragEvent, index: usize| {
		// set the index of the item being hovered over
		index_being_hovered.set(index as i32);
	};

	let on_drag_over = move |e: DragEvent| {
		e.prevent_default();
	};

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
						move || status.with(|status| status.queue.iter()
							.enumerate()
							.map(|(index, song)| view! {
								<div class="queue-item"
									draggable="true"
									on:dragstart=move |e: DragEvent| on_drag_start(e, index)
									on:drop=on_drop
									on:dragenter=move |e: DragEvent| on_drag_enter(e, index)
									on:dragover=on_drag_over
								>
									<Song song_image_path=song.image_path.clone() song_title=song.title.clone() song_artist=Artist::display_list(&song.artists) />
									<Show
										when=move || index != 0
										fallback=|| view!{
											<p>Playing</p>
										}>
										<button on:click=move |_| remove_song(index) on:mousedown=prevent_focus>
											<Icon class="remove-song" width=RM_BTN_SIZE height=RM_BTN_SIZE icon=icondata::CgTrash />
										</button>
									</Show>
								</div>
							})
							.collect::<Vec<_>>())
					}
				</ul>
			</div>
		</Show>

	}
}
