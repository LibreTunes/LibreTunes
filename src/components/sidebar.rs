use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use crate::components::playlists::Playlists;


#[component]
pub fn Sidebar(setter: WriteSignal<bool>, active: ReadSignal<bool>, logged_in:ReadSignal<bool>) -> impl IntoView {
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
            <div class="sidebar-bottom-container">
                <Show 
                    when=move || {logged_in() == true}
                    fallback=move|| view!{<h1>LOG IN PLEASE</h1>}
                >
                    <Playlists />
                </Show>
            </div>
        </div>
    }
}

