use leptos::*;
use leptos::leptos_dom::*;
use leptos_icons::AiIcon::*;
use leptos_icons::OcIcon::*;
use leptos_icons::*;

#[component]
pub fn Sidebar(setter: WriteSignal<bool>) -> impl IntoView {
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
                <div class="buttons" on:click=open_dashboard>
                    <Icon icon=Icon::from(OcHomeFillLg) />
                    <h1>Dashboard</h1>
                </div>
                <div class="buttons" on:click=open_search>
                    <Icon icon=Icon::from(AiSearchOutlined) />
                    <h1>Search</h1>
                </div>
            </div>
            <div class="sidebar-header">
                <h1>LibreTunes</h1>
            </div>

        </div>
    }
}
