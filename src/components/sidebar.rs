use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::BiIcon::*;
use leptos_icons::OcIcon::*;
use leptos_icons::*;

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
                    <Icon icon=Icon::from(OcHomeFillLg) />
                    <h1>Dashboard</h1>
                </div>
                <div class="buttons" on:click=open_search style={move || if !active() {"color: #e1e3e1"} else {""}}>
                    <Icon icon=Icon::from(BiSearchRegular) />
                    <h1>Search</h1>
                </div>
            </div>
            <div class="sidebar-bottom-container">
                <h1>LibreTunes</h1>
            </div>

        </div>
    }
}
