use leptos::prelude::*;
use leptos_icons::*;
use crate::components::upload_dropdown::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuEntry {
    Dashboard,
    Search,
}

impl MenuEntry {
    pub const fn path(&self) -> &'static str {
        match self {
            MenuEntry::Dashboard => "/dashboard",
            MenuEntry::Search => "/search",
        }
    }

    pub const fn icon(&self) -> icondata::Icon {
        match self {
            MenuEntry::Dashboard => icondata::OcHomeFillLg,
            MenuEntry::Search => icondata::BiSearchRegular,
        }
    }

    pub const fn title(&self) -> &'static str {
        match self {
            MenuEntry::Dashboard => "Dashboard",
            MenuEntry::Search => "Search",
        }
    }

    pub const fn all() -> [MenuEntry; 2] {
        [
            MenuEntry::Dashboard,
            MenuEntry::Search,
        ]
    }
}

#[component]
pub fn MenuItem(entry: MenuEntry, #[prop(into)] active: Signal<bool>) -> impl IntoView {
    view! {
        <a class="menu-btn" href={entry.path().to_string()}
            style={move || if active() {"color: var(--color-menu-active);"} else {""}} 
        >
            <Icon height="1.7rem" width="1.7rem" icon={entry.icon()} {..} class="mr-2" />
            <h2>{entry.title()}</h2>
        </a>
    }
}

#[component]
pub fn Menu(upload_open: RwSignal<bool>, add_artist_open: RwSignal<bool>, add_album_open: RwSignal<bool>) -> impl IntoView {
    use leptos_router::hooks::use_location;
    let location = use_location();

    let active_entry = Signal::derive(move || {
        let path = location.pathname.get();
        MenuEntry::all().into_iter().find(|entry| entry.path() == path)
    });

    let dropdown_open = RwSignal::new(false);

    view! {
        <div class="home-card">
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
            <div class="flex">
                <h1 class="text-xl font-bold">"LibreTunes"</h1>
                <div class="upload-dropdown-container">
                    <UploadDropdownBtn dropdown_open=dropdown_open/>
                    <Show
                        when= move || dropdown_open()
                        fallback=move || view! {}
                    >
                        <UploadDropdown dropdown_open=dropdown_open upload_open=upload_open add_artist_open=add_artist_open add_album_open=add_album_open/>
                    </Show>
                </div>
            </div>
            {MenuEntry::all().into_iter().map(|entry| {
                let active = Signal::derive(move || active_entry.get() == Some(entry));
                view! { <MenuItem entry active /> }
            }).collect::<Vec<_>>()}
        </div>
    }
}
