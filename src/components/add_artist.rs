use leptos::*;
use leptos::leptos_dom::log;
use leptos_icons::*;
use crate::api::artists::add_artist;

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
#[component]
pub fn AddArtist(open: RwSignal<bool>) -> impl IntoView {
    let close_dialog = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		open.set(false);
	};


    view! {
        <Show when=open fallback=move|| view!{}>
            <div class="add-artist-container">
                <div class="upload-header">
                    <h1>Add Artist</h1>
                </div>
                <div class="close-button" on:click=close_dialog><Icon icon=icondata::IoClose /></div>
                <form class="create-artist-form" action="POST">
                    <div class="input-bx">
                        <input type="text" name="title" required class="text-input" required/>
                        <span>Artist Name</span>
                    </div>
                    <button type="submit" class="upload-button">Add</button>
                </form>
            </div>
        </Show>
    }
}