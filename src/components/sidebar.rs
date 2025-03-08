use leptos::prelude::*;
use leptos_icons::*;
use crate::components::upload_dropdown::*;

#[component]
pub fn Sidebar(upload_open: RwSignal<bool>, add_artist_open: RwSignal<bool>, add_album_open: RwSignal<bool>) -> impl IntoView {
    use leptos_router::hooks::use_location;
    let location = use_location();

    let dropdown_open = RwSignal::new(false);

    let on_dashboard = Signal::derive(
        move || location.pathname.get().starts_with("/dashboard") || location.pathname.get() == "/",
    );

    let on_search = Signal::derive(
        move || location.pathname.get().starts_with("/search"),
    );

    view! {
        <div class="sidebar-container">
            <div class="sidebar-top-container">
                <Show
                    when=move || {upload_open.get() || add_artist_open.get() || add_album_open.get()}
                    fallback=move || view! {}
                >
                    <div class="upload-overlay" on:click=move |_| {
                        upload_open.set(false);
                        add_artist_open.set(false);
                        add_album_open.set(false);
                    }></div>
                </Show>
                <h2 class="header">LibreTunes</h2>
                <div class="upload-dropdown-container">
                    <UploadDropdownBtn dropdown_open=dropdown_open/>
                    <Show
                        when= move || dropdown_open()
                        fallback=move || view! {}
                    >
                        <UploadDropdown dropdown_open=dropdown_open upload_open=upload_open add_artist_open=add_artist_open add_album_open=add_album_open/>
                    </Show>
                </div>
                <a class="buttons" href="/dashboard" style={move || if on_dashboard() {"color: #e1e3e1"} else {""}} >
                    <Icon icon={icondata::OcHomeFillLg} />
                    <h1>Dashboard</h1>
                </a>
                <a class="buttons" href="/search" style={move || if on_search() {"color: #e1e3e1"} else {""}}>
                    <Icon icon={icondata::BiSearchRegular} />
                    <h1>Search</h1>
                </a>
            </div>
            <Bottom />

        </div>
    }
}

#[component]
pub fn Bottom() -> impl IntoView {
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
