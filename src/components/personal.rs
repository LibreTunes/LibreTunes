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
    let logged_in = create_rw_signal(false);
    let user_signal = create_rw_signal(User::default());

    let open_dropdown = move |_| {
        spawn_local(async move {
            let user = get_user().await;
            if let Ok(user) = user {
                logged_in.set(true);
                user_signal.update(|value| {
                    *value = user;
                });
            } else {
                logged_in.set(false);
            }
            set_dropdown_open.update(|value| *value = !*value);
        });
    };

    view! {
        <div class="profile-container">
            <div class="profile-icon" on:click=open_dropdown>
                <Icon icon=icondata::CgProfile />
            </div>
            <div class="dropdown-container" style={move || if dropdown_open() {"display: flex"} else {"display: none"}}>
                <Show
                    when=logged_in
                    fallback=|| view!{
                        <DropDownNotLoggedIn />
                    }>
                    <DropDownLoggedIn user_signal=user_signal logged_in=logged_in></DropDownLoggedIn>
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
pub fn DropDownLoggedIn(user_signal: RwSignal<User>, logged_in: RwSignal<bool>) -> impl IntoView {

    let logout = move |_| {
        spawn_local(async move {
            let result = logout().await;
            if let Err(err) = result {
                log!("Error logging out: {:?}", err);
            } else {
                log!("Logged out successfully");
                user_signal.update(|value| *value = User::default());
                logged_in.set(false);
            }
        });
    };

    view! {
        <div class="dropdown-logged">
            <h1>"Logged In"</h1>
            <div class="profile-info">
                <h1>{move || user_signal.with(|user| user.username.clone())}</h1>
            </div>
            <button on:click=logout class="auth-button">Log Out</button>
        </div>
    }
}
