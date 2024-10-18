use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use crate::auth::get_user;
use crate::auth::logout;
use crate::models::User;

#[component]
pub fn Personal() -> impl IntoView {
    view! {
        <div class=" personal-container">
            <Profile />
        </div>
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = create_signal(false);
    let user = create_local_resource(move || dropdown_open.get(), |_| async move { get_user().await });
    let logged_in = create_local_resource(move || dropdown_open.get(), |_| async move { get_user().await.is_ok() });

    let open_dropdown = move |_| {
        set_dropdown_open.update(|value| *value = !*value);
    };

    view! {
        <div class="profile-container">
            <div class="profile-icon" on:click=open_dropdown>
                <Icon icon=icondata::CgProfile />
            </div>
            <div class="dropdown-container" style={move || if dropdown_open() {"display: flex"} else {"display: none"}}>
                <Show
                    when=move || logged_in.get().unwrap_or_default()
                    fallback=|| view!{
                        <DropDownNotLoggedIn />
                    }>
                    <DropDownLoggedIn logged_in=logged_in />
                </Show>
            </div>
        </div>
    }
}
#[component]
pub fn DropDownNotLoggedIn() -> impl IntoView {
    view! {
        <div class="dropdown-logged">
            <h1>Not Logged In</h1>
            <a href="/login"><button class="auth-button">Log In</button></a>
            <a href="/signup"><button class="auth-button">Sign Up</button></a>
        </div>
    }
}
#[component]
pub fn DropDownLoggedIn(logged_in: Resource<bool, bool>) -> impl IntoView {

    let logout = move |_| {
        spawn_local(async move {
            let result = logout().await;
            if let Err(err) = result {
                log!("Error logging out: {:?}", err);
            } else {
                log!("Logged out successfully");
                logged_in.set(false);
            }
        });
    };

    view! {
        <div class="dropdown-logged">
            <h1>"Logged In"</h1>
            <button on:click=logout class="auth-button">Log Out</button>
        </div>
    }
}
