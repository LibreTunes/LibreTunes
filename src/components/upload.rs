use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::Form;
use crate::search::search_artists;
use crate::models::Artist;

#[component]
pub fn UploadBtn(dialog_open: RwSignal<bool>) -> impl IntoView {
	let open_dialog = move |_| {
		dialog_open.set(true);
	};

	view! {
		<button class="upload-btn" on:click=open_dialog>
			<div class="add-sign">
				<Icon icon=icondata::IoAddSharp />
			</div>
			Upload
		</button>
	}
}

#[component]
pub fn Upload(open: RwSignal<bool>) -> impl IntoView {
	// Create signals for the artist input and the filtered artists
	let (artists, set_artists) = create_signal("".to_string());
	let (filtered_artists, set_filtered_artists)  = create_signal(vec![]);

	let close_dialog = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		open.set(false);
	};
	// Create a filter function to handle filtering artists
	let handle_filter = move |ev: leptos::ev::Event| {
		ev.prevent_default();

		let artist_input: String = event_target_value(&ev);

		//Get the artist that we are currently searching for
		let mut all_artists: Vec<&str> = artist_input.split(",").collect();
		let search = all_artists.pop().unwrap().to_string();
		
		//Update the artist signal with the input
		set_artists.update(|value: &mut String| *value = artist_input);

		spawn_local(async move {
			let filter_results = search_artists(search, 3).await;
			if let Err(err) = filter_results {
				log!("Error filtering artists: {:?}", err);
			} else if let Ok(artists) = filter_results {
				log!("Filtered artists: {:?}", artists);

				set_filtered_artists.update(|value| *value = artists);
			}
		})
	};
	view! {
		<Show when=open fallback=move || view! {}>
			<div class="upload-container" open=open>
				<div class="close-button" on:click=close_dialog><Icon icon=icondata::IoClose /></div>
				<div class="upload-header">
					<h1>Upload Song</h1>
				</div>
				<Form action="/api/upload" method="POST" enctype=String::from("multipart/form-data") class="upload-form">
					<div class="input-bx">
						<input type="text" name="title" required class="text-input" required/>
						<span>Title</span>
					</div>
					<div class="artists">
						<div class="input-bx">
							<input type="text" name="artist_ids" class="text-input" prop:value=artists required on:input=handle_filter/>
							<span>Artists</span>
						</div>
						<Show
							when=move || {filtered_artists.get().len() > 0}
							fallback=move || view! {}						
						>
							<ul class="artist_results">
								{
									move || filtered_artists.get().iter().enumerate().map(|(_index,filtered_artist)| view! {
										<Artist artist=filtered_artist.clone() artists=artists set_artists=set_artists set_filtered=set_filtered_artists/>
									}).collect::<Vec<_>>()
								}
							</ul>
						</Show>
					</div>
					<div class="input-bx">
						<input type="text" name="album_id" class="text-input" required/>
						<span>Album ID</span>
					</div>
					<div class="input-bx">
						<input type="number" name="track_number" class="text-input" required/>
						<span>Track Number</span>
					</div>
					<div class="release-date">
						<div class="left">
							<span>Release</span>
							<span>Date</span>
						</div>
						<input class="info" type="date" name="release_date"/>
					</div>
					<div class="file">
						<span>File</span>
						<input class="info" type="file" name="file"/>
					</div>
					<button type="submit" class="upload-button">Upload</button>
				</Form>
			</div>
		</Show>
	}
}

#[component]
pub fn Artist(artist: Artist, artists: ReadSignal<String>, set_artists: WriteSignal<String>, set_filtered: WriteSignal<Vec<Artist>>) -> impl IntoView {

	// Create a function to add an artist to the artist input
	let add_artist = move |_| {
		//Create an empty string to hold the artist ids
		let mut s: String = String::from("");
		//Get the current value of the artist input
		let all_artirts: String = artists.get();
		//Split the input into a vector of artists separated by commas
		let mut ids: Vec<&str> = all_artirts.split(",").collect();
		//If there is only one artist in the input, get their id equivalent and add it to the string
		if ids.len() == 1 {
			let value_str = match artist.id.clone() {
				Some(v) => v.to_string(),
				None => String::from("None"),
			};
			s.push_str(&value_str);
			s.push_str(",");
			set_artists.update(|value| *value = s);
		//If there are multiple artists in the input, pop the last artist by string off the vector, 
		//get their id equivalent, and add it to the string
		} else {
			ids.pop();
			for id in ids {
				s.push_str(id);
				s.push_str(",");
			}
			let value_str = match artist.id.clone() {
				Some(v) => v.to_string(),
				None => String::from("None"),
			};
			s.push_str(&value_str);
			s.push_str(",");
			set_artists.update(|value| *value = s);
		}
		//Clear the search results
		set_filtered.update(|value| *value = vec![]);
	};

	view! {
		<div class="artist" on:click=add_artist>
			{artist.name.clone()}
		</div>
	}
}