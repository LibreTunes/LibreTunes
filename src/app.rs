use crate::playbar::PlayBar;
use crate::playbar::CustomTitle;
use crate::queue::Queue;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::login::*;
use crate::pages::signup::*;
use crate::pages::profile::*;
use crate::pages::albumpage::*;
use crate::pages::artist::*;
use crate::pages::songpage::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::util::state::GlobalState;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_context(GlobalState::new());

    let upload_open = create_rw_signal(false);
    let add_artist_open = create_rw_signal(false);
    let add_album_open = create_rw_signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/libretunes.css"/>

        // sets the document title
        <CustomTitle />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=move || view! { <HomePage upload_open=upload_open add_artist_open=add_artist_open add_album_open=add_album_open/> }>
                        <Route path="" view=Dashboard />
                        <Route path="dashboard" view=Dashboard />
                        <Route path="search" view=Search />
                        <Route path="user/:id" view=Profile />
                        <Route path="user" view=Profile />
                        <Route path="album/:id" view=AlbumPage />
                        <Route path="artist/:id" view=ArtistPage />
                        <Route path="song/:id" view=SongPage />
                    </Route>
                    <Route path="/login" view=Login />
                    <Route path="/signup" view=Signup />
                </Routes>
            </main>
        </Router>
    }
}

use crate::components::sidebar::*;
use crate::components::dashboard::*;
use crate::components::search::*;
use crate::components::personal::Personal;
use crate::components::upload::*;
use crate::components::add_artist::AddArtist;
use crate::components::add_album::AddAlbum;

/// Renders the home page of your application.
#[component]
fn HomePage(upload_open: RwSignal<bool>, add_artist_open: RwSignal<bool>, add_album_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="home-container">
            <Upload open=upload_open/>
            <AddArtist open=add_artist_open/>
            <AddAlbum open=add_album_open/>
            <Sidebar upload_open=upload_open add_artist_open=add_artist_open add_album_open=add_album_open/>
            // This <Outlet /> will render the child route components
            <Outlet />
            <Personal />
            <PlayBar />
            <Queue />
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_axum::ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
