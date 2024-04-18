use leptos::*;
use leptos_icons::*;
use leptos::leptos_dom::*;
use crate::api::playlists::create_playlist;

#[component]
pub fn CreatePlayList(opened: ReadSignal<bool>,closer: WriteSignal<bool>) -> impl IntoView {

    let (playlist_name, set_playlist_name) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let new_playlist_name = playlist_name.get();
        spawn_local(async move {
            let create_result = create_playlist(new_playlist_name).await;
            if let Err(err) = create_result {
                // Handle the error here, e.g., log it or display to the user
                log!("Error creating playlist: {:?}", err);
            } else {
                log!("Playlist created successfully!");
                closer.update(|value| *value = false);
            }
        })
    }; 

    view! {
        <div class="create-playlist-popup-container" style={move || if opened() {"display:flex"} else {"display:none"}}>
            <div class="close-button" on:click=move |_| closer.update(|value| *value = false)>
                <Icon icon=icondata::IoCloseSharp />
            </div>
            <h1 class="header">Create Playlist</h1>    
            <form class="create-playlist-form" action="POST" on:submit=on_submit>
                <input class="name-input"  type="text" placeholder="Playlist Name" 
                    on:input=move |ev| {
                        set_playlist_name(event_target_value(&ev));
                        log!("playlist name changed to: {}", playlist_name.get());
                    }
                    prop:value=playlist_name
                />
                <button class="create-button" type="submit">Create</button>
            </form>
        </div>
    }
}