use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    use leptos_router::use_location;
    let location = use_location();

    let on_dashboard = Signal::derive(
        move || location.pathname.get().starts_with("/dashboard") || location.pathname.get() == "/",
    );

    let on_search = Signal::derive(
        move || location.pathname.get().starts_with("/search"),
    );

    view! {
        <div class="sidebar-container">
            <div class="sidebar-top-container">
                <h2 class="header">LibreTunes</h2>
                <a class="buttons" href="/dashboard" style={move || if on_dashboard() {"color: #e1e3e1"} else {""}} >
                    <Icon icon=icondata::OcHomeFillLg />
                    <h1>Dashboard</h1>
                </a>
                <a class="buttons" href="/search" style={move || if on_search() {"color: #e1e3e1"} else {""}}>
                    <Icon icon=icondata::BiSearchRegular />
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
