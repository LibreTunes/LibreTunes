use leptos::*;
use leptos_icons::*;

#[component]
pub fn AddArtistBtn(add_artist_open: RwSignal<bool>) -> impl IntoView {
    let open_dialog = move |_| {
        add_artist_open.set(true);
    };

    view! {
        <button class="add-artist-btn add-btns" on:click=open_dialog>
            Add Artist
        </button>
    }
}