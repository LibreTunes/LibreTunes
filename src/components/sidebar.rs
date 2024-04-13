use leptos::ev::play;
use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use crate::api::playlists::create_playlist;

#[component]
pub fn Sidebar(setter: WriteSignal<bool>, active: ReadSignal<bool>) -> impl IntoView {
    let open_dashboard = move |_| {
        setter.update(|value| *value = true);
        log!("open dashboard");
    };
    let open_search = move |_| {
        setter.update(|value| *value = false);
        log!("open search");
    };

    view! {
        <div class="sidebar-container">
            <div class="sidebar-top-container">
                <h2 class="header">LibreTunes</h2>
                <div class="buttons" on:click=open_dashboard style={move || if active() {"color: #e1e3e1"} else {""}} >
                    <Icon icon=icondata::OcHomeFillLg />
                    <h1>Dashboard</h1>
                </div>
                <div class="buttons" on:click=open_search style={move || if !active() {"color: #e1e3e1"} else {""}}>
                    <Icon icon=icondata::BiSearchRegular />
                    <h1>Search</h1>
                </div>
            </div>
            <Bottom />

        </div>
    }
}

#[component]
pub fn Bottom() -> impl IntoView {
    let (create_playlist_open, set_create_playlist_open) = create_signal(false);

    view! {
        <div class="sidebar-bottom-container">
            <div class="heading">
                <h1 class="header">Playlists</h1>
                <button on:click=move|_| set_create_playlist_open.update(|value|*value = true) class="add-playlist">
                    <div class="add-sign">
                        <Icon icon=icondata::IoAddSharp />
                    </div>
                    New Playlist
                </button>
            </div>
            <CreatePlayList opened=create_playlist_open closer=set_create_playlist_open/>
        </div>
    }
}

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