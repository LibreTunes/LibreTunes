use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::login::*;
use crate::pages::signup::*;
use crate::playbar::PlayBar;
use crate::playstatus::PlayStatus;
use crate::queue::Queue;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::leptos_dom::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/libretunes.css"/>

        // sets the document title
        <Title text="LibreTunes"/>

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
                    <Route path="" view=HomePage/>
                    <Route path="/login" view=Login />
                    <Route path="/signup" view=Signup />
                </Routes>
            </main>
        </Router>
    }
}

use crate::components::dashboard::*;
use crate::components::personal::*;
use crate::components::search::*;
use crate::components::sidebar::*;

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use crate::auth::check_auth;

    let play_status = PlayStatus::default();
    let play_status = create_rw_signal(play_status);

    let (logged_in, set_logged_in) = create_signal(false);

    let (dashboard_open, set_dashboard_open) = create_signal(true);

    create_effect(move |_| {
        spawn_local(async move {
            let auth_result = check_auth().await;
            if let Err(err) = auth_result {
                log!("Error checking auth: {:?}", err);
            } else if let Ok(true) = auth_result {
                log!("User is logged in");
                set_logged_in.update(|value| *value = true);
            } else if let Ok(false) = auth_result {
                log!("User is not logged in");
                set_logged_in.update(|value| *value = false);
            }
            
        });
    });

    view! {
        <div class="home-container">
            <Sidebar setter=set_dashboard_open active=dashboard_open logged_in=logged_in/>
            <Show
                when=move || {dashboard_open() == true}
                fallback=move || view! { <Search /> }
            >
                <Dashboard />
            </Show>
            <Personal logged_in=logged_in/>
            <PlayBar status=play_status/>
            <Queue status=play_status/>
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
