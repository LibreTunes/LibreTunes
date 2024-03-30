use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use leptos_icons::CgIcon::*;

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

    let open_dropdown = move |_| {
        set_dropdown_open.update(|value| *value = !*value);
        log!("opened dropdown");
    };
    view! {
        <div class="profile-container">
            <div class="profile-icon" on:click=open_dropdown>
                <Icon icon=Icon::from(CgProfile) />
            </div>
            <div class="dropdown-container" style={move || if dropdown_open() {"display: flex"} else {"display: none"}}>
                <DropDownNotLoggedIn />
            </div>
        </div>
    }
}
#[component]
pub fn DropDownNotLoggedIn() -> impl IntoView {
    view! {
        <div class="dropdown-not-logged">
            <h1>Not Logged in!</h1>
            <a href="/login"><button class="auth-button">Log In</button></a>
            <a href="/signup"><button class="auth-button">Sign up</button></a>
        </div>
    }
}
