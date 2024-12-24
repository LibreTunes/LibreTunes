use leptos::prelude::*;
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
    let artist_name = create_rw_signal("".to_string());

    let close_dialog = move |ev: leptos::ev::MouseEvent| {
		ev.prevent_default();
		open.set(false);
	};
    let on_add_artist = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let artist_name_clone = artist_name.get();
        spawn_local(async move {
            let add_artist_result = add_artist(artist_name_clone).await;
            if let Err(err) = add_artist_result {
                log!("Error adding artist: {:?}", err);
            } else if let Ok(artist) = add_artist_result {
                log!("Added artist: {:?}", artist);
                artist_name.set("".to_string());
            }
        })
    };

    view! {
        <Show when=open fallback=move|| view!{}>
            <div class="add-artist-container">
                <div class="upload-header">
                    <h1>Add Artist</h1>
                </div>
                <div class="close-button" on:click=close_dialog><Icon icon={icondata::IoClose} /></div>
                <form class="create-artist-form" action="POST" on:submit=on_add_artist>
                    <div class="input-bx">
                        <input type="text" name="title" required class="text-input" 
                            prop:value=artist_name
                            on:input=move |ev: leptos::ev::Event| {
                                artist_name.set(event_target_value(&ev));
                            }        
                            required 
                         />
                        <span>Artist Name</span>
                    </div>
                    <button type="submit" class="upload-button">Add</button>
                </form>
            </div>
        </Show>
    }
}