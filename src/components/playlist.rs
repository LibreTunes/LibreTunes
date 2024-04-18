use leptos::*;
use leptos_icons::*;
use crate::models::Playlist;

#[component]
pub fn Playlist(playlist: Playlist) -> impl IntoView {
    let (show_playlist, set_show_playlist) = create_signal(false);

    view! {
        <div class="playlist" on:click=move|_| set_show_playlist.update(|value| *value=true) >
            <h1 class="name">{playlist.name.clone()}</h1>
            <Show
                when=move || show_playlist()
                fallback=move || view! {<div></div>}
            >
                <div class="playlist-container">
                    <div class="close-button" on:click=move |_| set_show_playlist.update(|value| *value = false)>
                        <Icon icon=icondata::IoCloseSharp />
                    </div>
                    <h1>{playlist.name.clone()}</h1>
                </div>
            </Show>
        </div>
    }
}