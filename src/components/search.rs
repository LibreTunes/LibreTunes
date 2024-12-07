use html::Li;
use leptos::*;
use crate::search::search_albums;
use crate::search::search_artists;
use crate::search::search_songs;
use crate::song::Song;
use crate::models::Album;
use crate::models::Artist;
use crate::models::Song;
use crate::util::state::GlobalState;
use leptos::ev::*;
use leptos::leptos_dom::*;
use leptos_icons::*;

const OPTIONS_BTN_SIZE: &str = "2.5rem";

#[component]
pub fn Search() -> impl IntoView {

	let status = GlobalState::play_status();

    let search_query = create_rw_signal(String::new());
    let album_search_results = create_rw_signal(Vec::<(Album, f32)>::new());
	let artist_search_results = create_rw_signal(Vec::<(Artist, f32)>::new());
	let song_search_results = create_rw_signal(Vec::<(Song, f32)>::new());
    let search_limit = 10;

    let on_input = move |e: Event| {
        search_query.set(event_target_value(&e));

        log!("Search Query: {:?}", search_query.get_untracked());

        spawn_local(async move {
            log!("Searching for: {:?}", search_query.get_untracked());
			let albums = search_albums(search_query.get_untracked(), search_limit).await;
			let artists = search_artists(search_query.get_untracked(), search_limit).await;
			let songs = search_songs(search_query.get_untracked(), search_limit).await;

			match albums {
				Ok(albums) => {
					album_search_results.set(albums);
				}
				Err(err) => {
					log!("Error searching albums: {:?}", err);
				}
			}

			match artists {
				Ok(artists) => {
					artist_search_results.set(artists);
				}
				Err(err) => {
					log!("Error searching artists: {:?}", err);
				}
			}

			match songs {
				Ok(songs) => {
					song_search_results.set(songs);
				}
				Err(err) => {
					log!("Error searching songs: {:?}", err);
				}
			}
        });
    };

    let on_disabled = move |_e: FocusEvent| {
		status.update(|status| {
			status.search_active = false;
		});
        log!("Search Bar Disabled");
    };

    let on_enabled = move |_e: FocusEvent| {
        status.update(|status| {
            status.search_active = true;
        });
        log!("Search Bar Enabled");
    };

	let prevent_focus = move |e: MouseEvent| {
		e.prevent_default();
	};

    view! {
		<div class="search-container home-component">
			<div class="search-bar">
				<input type="search" placeholder="Search" on:input=on_input on:blur=on_disabled on:focus=on_enabled/>
			</div>
			<div class="search-results">
				// Display 3 columns of search results: songs, albums, and artists
				<ul class="search-result-list">
					{move || song_search_results.with(|songs| -> Vec<leptos::HtmlElement<Li>> {
						let mut song_list = Vec::new();
						log!("Songs: {:?}", songs);
						for (song, _) in songs {
							song_list.push(view! {
								<li class="search-result">
									<div class="result-container">
										<Song song_image_path=match song.image_path.clone() {
											Some(path) => path,
											None => "".to_string()
										} song_title=song.title.clone() song_artist="".to_string() />
										<div class="right-side-result">
											<div class="search-item-type">
												"(Song)"
											</div>
											<button class="search-result-options" on:mousedown=prevent_focus>
												<Icon class="search-result-options-icon" width=OPTIONS_BTN_SIZE height=OPTIONS_BTN_SIZE icon=icondata::BsThreeDotsVertical />
											</button>
										</div>
									</div>
								</li>
							});
						}
						song_list
					})}
				</ul>
				<ul class="search-result-list">
					{move || album_search_results.with(|albums| -> Vec<leptos::HtmlElement<Li>> {
						let mut album_list = Vec::new();
						log!("Albums: {:?}", albums);
						for (album, _) in albums {
							album_list.push(view! {
								<li class="search-result">
									<div class="result-container">
										<div class="search-result-album">
											{album.title.clone()}
											{match album.release_date {
												Some(date) => format!(" ({})", date),
												None => "".to_string()
											}}
										</div>
										<div class="right-side-result">
											<div class="search-item-type">
												"(Album)"
											</div>
											<button class="search-result-options" on:mousedown=prevent_focus>
												<Icon class="search-result-options-icon" width=OPTIONS_BTN_SIZE height=OPTIONS_BTN_SIZE icon=icondata::BsThreeDotsVertical />
											</button>
										</div>
									</div>
								</li>
							});
						}
						album_list
					})}
				</ul>
				<ul class="search-result-list">
					{move || artist_search_results.with(|artists| -> Vec<leptos::HtmlElement<Li>> {
						let mut artist_list = Vec::new();
						log!("Artists: {:?}", artists);
						for (artist, _) in artists {
							artist_list.push(view! {
								<li class="search-result">
									<div class="result-container">
										<div class="search-result-artist">
											{artist.name.clone()}
										</div>
										<div class="right-side-result">
											<div class="search-item-type">
												"(Artist)"
											</div>
											<button class="search-result-options" on:mousedown=prevent_focus>
												<Icon class="search-result-options-icon" width=OPTIONS_BTN_SIZE height=OPTIONS_BTN_SIZE icon=icondata::BsThreeDotsVertical />
											</button>
										</div>
									</div>
								</li>
							});
						}
						artist_list
					})}
				</ul>
			</div>
		</div>
	}
}