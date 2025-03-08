use leptos::prelude::*;
use leptos_icons::*;
use crate::components::menu::*;

#[component]
pub fn Sidebar(upload_open: RwSignal<bool>, add_artist_open: RwSignal<bool>, add_album_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <Menu upload_open add_artist_open add_album_open />
            <Playlists />
        </div>
    }
}

#[component]
pub fn Playlists() -> impl IntoView {
    view! {
        <div class="home-card">
            <div class="flex">
                <h1 class="header">Playlists</h1>
                <button class="add-playlist">
                    <div class="add-sign">
                        <Icon icon={icondata::IoAddSharp} />
                    </div>
                    New Playlist
                </button>
            </div>
        </div>
    }
}
