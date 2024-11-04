use crate::playbar::PlayBar;
use crate::playbar::CustomTitle;
use crate::playstatus::PlayStatus;
use crate::queue::Queue;
use leptos::*;
use leptos::logging::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::login::*;
use crate::pages::signup::*;
use crate::pages::profile::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::auth::get_logged_in_user;
use crate::models::User;

pub type LoggedInUserResource = Resource<(), Option<User>>;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let play_status = PlayStatus::default();
    let play_status = create_rw_signal(play_status);
    let upload_open = create_rw_signal(false);

    // A resource that fetches the logged in user
    // This will not automatically refetch, so any login/logout related code
    // should call `refetch` on this resource
    let logged_in_user: LoggedInUserResource = create_resource(|| (), |_| async {
        get_logged_in_user().await
            .inspect_err(|e| {
                error!("Error getting logged in user: {:?}", e);
            })
            .ok()
            .flatten()
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/libretunes.css"/>

        // sets the document title
        <CustomTitle play_status=play_status/>

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
                    <Route path="" view=move || view! { <HomePage play_status=play_status upload_open=upload_open/> }>
                        <Route path="" view=Dashboard />
                        <Route path="dashboard" view=Dashboard />
                        <Route path="search" view=Search />
                        <Route path="user/:id" view=move || view!{ <Profile logged_in_user /> } />
                        <Route path="user" view=move || view!{ <Profile logged_in_user /> } />
                    </Route>
                    <Route path="/login" view=move || view!{ <Login user=logged_in_user /> } />
                    <Route path="/signup" view=move || view!{ <Signup user=logged_in_user /> } />
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

/// Renders the home page of your application.
#[component]
fn HomePage(play_status: RwSignal<PlayStatus>, upload_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="home-container">
            <Upload open=upload_open/>
            <Sidebar upload_open=upload_open/>
            // This <Outlet /> will render the child route components
            <Outlet />
            <Personal />
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
