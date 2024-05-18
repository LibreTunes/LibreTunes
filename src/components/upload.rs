use std::rc::Rc;
use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::Form;
use web_sys::Response;
use crate::search::search_artists;
use crate::search::search_albums;
use crate::models::Artist;
use crate::models::Album;

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

	let (albums, set_albums) = create_signal("".to_string());
	let (filtered_albums, set_filtered_albums) = create_signal(vec![]);

	let (error_msg, set_error_msg) = create_signal::<Option<String>>(None);

	let close_dialog = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		open.set(false);
	};
	// Create a filter function to handle filtering artists
	// Allow users to search for artists by name, converts the artist name to artist id to be handed off to backend
	let handle_filter_artists = move |ev: leptos::ev::Event| {
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
	// Create a filter function to handle filtering albums
	// Allow users to search for albums by title, converts the album title to album id to be handed off to backend
	let handle_filter_albums = move |ev: leptos::ev::Event| {
		ev.prevent_default();

		let album_input: String = event_target_value(&ev);
		
		//Update the album signal with the input
		set_albums.update(|value: &mut String| *value = album_input);

		spawn_local(async move {
			let filter_results = search_albums(albums.get_untracked(), 3).await;
			if let Err(err) = filter_results {
				log!("Error filtering albums: {:?}", err);
			} else if let Ok(albums) = filter_results {
				log!("Filtered albums: {:?}", albums);
				set_filtered_albums.update(|value| *value = albums);
			}
		})
	};

	let handle_response = Rc::new(move |response: &Response| {
		if response.ok() {
			set_error_msg.update(|value| *value = None);
			open.set(false);
		} else {
			// TODO: Extract error message from response
			set_error_msg.update(|value| *value = Some("Error uploading song".to_string()));
		}
	});

	view! {
		<Show when=open fallback=move || view! {}>
			<div class="upload-container" open=open>
				<div class="close-button" on:click=close_dialog><Icon icon=icondata::IoClose /></div>
				<div class="upload-header">
					<h1>Upload Song</h1>
				</div>
				<Form action="/api/upload" method="POST" enctype=String::from("multipart/form-data")
					class="upload-form" on_response=handle_response.clone()>
					<div class="input-bx">
						<input type="text" name="title" required class="text-input" required/>
						<span>Title</span>
					</div>
					<div class="artists has-search">
						<div class="input-bx">
							<input type="text" name="artist_ids" class="text-input" prop:value=artists on:input=handle_filter_artists/>
							<span>Artists</span>
						</div>
						<Show
							when=move || {filtered_artists.get().len() > 0}
							fallback=move || view! {}						
						>
							<ul class="artist_results search-results">
								{
									move || filtered_artists.get().iter().enumerate().map(|(_index,filtered_artist)| view! {
										<Artist artist=filtered_artist.clone() artists=artists set_artists=set_artists set_filtered=set_filtered_artists/>
									}).collect::<Vec<_>>()
								}
							</ul>
						</Show>
					</div>
					<div class="albums has-search">
						<div class="input-bx">
							<input type="text" name="album_id" class="text-input" prop:value=albums on:input=handle_filter_albums/>
							<span>Album ID</span>
						</div>
						<Show
							when=move || {filtered_albums.get().len() > 0}
							fallback=move || view! {}						
						>
							<ul class="album_results search-results">
								{
									move || filtered_albums.get().iter().enumerate().map(|(_index,filtered_album)| view! {
										<Album album=filtered_album.clone() _albums=albums set_albums=set_albums set_filtered=set_filtered_albums/>
									}).collect::<Vec<_>>()
								}
							</ul>
						</Show>
					</div>
					
					<div class="input-bx">
						<input type="number" name="track_number" class="text-input"/>
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
						<input class="info" type="file" accept=".mp3" name="file" required/>
					</div>
					<button type="submit" class="upload-button">Upload</button>
				</Form>
				<Show
					when=move || {error_msg.get().is_some()}
					fallback=move || view! {}
				>
					<div class="error-msg">
						<Icon icon=icondata::IoAlertCircleSharp />
						{error_msg.get().as_ref().unwrap()}
					</div>
				</Show>
			</div>
		</Show>
	}
}

#[component]
pub fn Artist(artist: Artist, artists: ReadSignal<String>, set_artists: WriteSignal<String>, set_filtered: WriteSignal<Vec<Artist>>) -> impl IntoView {
	// Converts artist name to artist id and adds it to the artist input
	let add_artist = move |_| {
		//Create an empty string to hold previous artist ids
		let mut s: String = String::from("");
		//Get the current artist input
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
		<div class="artist result" on:click=add_artist>
			{artist.name.clone()}
		</div>
	}
}
#[component]
pub fn Album(album: Album, _albums: ReadSignal<String>, set_albums: WriteSignal<String>, set_filtered: WriteSignal<Vec<Album>>) -> impl IntoView {
	//Converts album title to album id to upload a song
	let add_album = move |_| {
		let value_str = match album.id.clone() {
			Some(v) => v.to_string(),
			None => String::from("None"),
		};
		set_albums.update(|value| *value = value_str);
		set_filtered.update(|value| *value = vec![]);
	};
	view! {
		<div class="album result" on:click=add_album>
			{album.title.clone()}
		</div>
	}
}