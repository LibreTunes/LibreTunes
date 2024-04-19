use leptos::*;
use leptos::leptos_dom::*;
use leptos_icons::*;
use crate::components::create_playlist::CreatePlayList;
use crate::components::playlist::Playlist;
use crate::api::playlists::get_playlists;

#[component]
pub fn Playlists() -> impl IntoView {
    let (create_playlist_open, set_create_playlist_open) = create_signal(false);
    let (playlists, set_playlists) = create_signal(vec![]);

    create_effect(move |_| {
        spawn_local(async move {
            let playlists = get_playlists().await;
            if let Err(err) = playlists {
                // Handle the error here, e.g., log it or display to the user
                log!("Error getting playlists: {:?}", err);
            } else {
                log!("Playlists: {:?}", playlists);
                set_playlists.update(|value| *value = playlists.unwrap());
            }
        })
    });
    
    view! {
        <div class="sidebar-playlists-container">
            <div class="heading">
                <h1 class="header">Playlists</h1>
                <button on:click=move|_| set_create_playlist_open.update(|value|*value = true) class="add-playlist">
                    <div class="add-sign">
                        <Icon icon=icondata::IoAddSharp />
                    </div>
                    New Playlist
                </button>
            </div>
            <Show
                when=move || create_playlist_open()
                fallback=move || view! {<div></div>}
            >
                <CreatePlayList closer=set_create_playlist_open/>
            </Show>
            
            <ul class="playlists">
                {
                    move || playlists.get().iter().enumerate().map(|(index,playlist)| view! {
                       <Playlist playlist=playlist.clone() />
                    }).collect::<Vec<_>>()
                }
            </ul>
            
        </div>
    }
}



