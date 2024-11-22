use leptos::*;
use crate::search::search;
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
    let search_results = create_rw_signal((Vec::<(Album, f32)>::new(), Vec::<(Artist, f32)>::new(), Vec::<(Song, f32)>::new()));
    let search_limit = 10;

    let on_input = move |e: Event| {
        search_query.set(event_target_value(&e));

        log!("Search Query: {:?}", search_query.get_untracked());

        if search_query.get_untracked().len() < 3 {
            search_results.set((Vec::<(Album, f32)>::new(), Vec::<(Artist, f32)>::new(), Vec::<(Song, f32)>::new()));
            return;
        }
        spawn_local(async move {
            log!("Searching for: {:?}", search_query.get_untracked());
            let results = search(search_query.get_untracked(), search_limit).await;
            match results {
                Ok((albums, artists, songs)) => {
                    search_results.set((albums, artists, songs));
                }
                Err(err) => {
                    log!("Error searching: {:?}", err);
                }
            }
        });
    };

    let on_disabled = move |_e: FocusEvent| {
        
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
				<ul class="search-results-list">
					{
						move || search_results.with(|(albums, artists, songs)| -> Vec<_> {
							let mut album_index = 0;
							let mut artist_index = 0;
							let mut song_index = 0;
							let mut views = Vec::new();
							while album_index < albums.len() || artist_index < artists.len() || song_index < songs.len() {
								const RM_BTN_SIZE: &str = "2.5rem";
								let album_score = if album_index < albums.len() { albums[album_index].1 } else { f32::MAX };
								let artist_score = if artist_index < artists.len() { artists[artist_index].1 } else { f32::MAX };
								let song_score = if song_index < songs.len() { songs[song_index].1 } else { f32::MAX };
								if artist_score <= album_score && artist_score <= song_score {
									let artist = &artists[artist_index].0;
									artist_index += 1;
									views.push(view! {
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
								else if album_score <= artist_score && album_score <= song_score {
									let album = &albums[album_index].0;
									album_index += 1;
									views.push(view! {
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
								else if song_score <= artist_score && song_score <= album_score {
									let song = &songs[song_index].0;
									song_index += 1;
									views.push(view! {
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
							}
							views
						})
					}
				</ul>
			</div>
		</div>
	}
}
