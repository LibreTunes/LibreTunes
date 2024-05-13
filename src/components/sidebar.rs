use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use crate::components::upload::*;

#[component]
pub fn Sidebar(setter: WriteSignal<bool>, active: ReadSignal<bool>, upload_open: RwSignal<bool>) -> impl IntoView {
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
                <UploadBtn dialog_open=upload_open />
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
    view! {
        <div class="sidebar-bottom-container">
            <div class="heading">
                <h1 class="header">Playlists</h1>
                <button class="add-playlist">
                    <div class="add-sign">
                        <Icon icon=icondata::IoAddSharp />
                    </div>
                    New Playlist
                </button>
            </div>
        </div>
    }
}
