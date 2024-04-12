use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Personal(logged_in: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class=" personal-container">
            <Profile logged_in=logged_in/>
        </div>
    }
}

#[component]
pub fn Profile(logged_in: ReadSignal<bool>) -> impl IntoView {

    let (dropdown_open, set_dropdown_open) = create_signal(false);

    let open_dropdown = move |_| {
        set_dropdown_open.update(|value| *value = !*value);
        log!("opened dropdown");
    };
    view! {
        <div class="profile-container">
            <div class="profile-icon" on:click=open_dropdown>
                <Icon icon=icondata::CgProfile />
            </div>
            <div class="dropdown-container" style={move || if dropdown_open() {"display: flex"} else {"display: none"}}>
            <Show 
                when=move || {logged_in() == true}
                fallback=move || view!{<DropDownNotLoggedIn />}
            >
                <DropDownLoggedIn/>
            </Show>
                
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
#[component]
pub fn DropDownLoggedIn() -> impl IntoView {
    use crate::auth::logout;

    let logout = move |_ev: leptos::ev::MouseEvent| {
        spawn_local(async move {
            let _logout_result = logout().await;
            if let Err(err) = _logout_result {
                log!("Error logging out: {:?}", err);
            } else {
                log!("Logged out Successfully!");
                leptos_router::use_navigate()("/login", Default::default());
            }
        
        });
    };
    view! {
        <div class="dropdown-logged-in">
            <h1>Logged in!</h1>
            <button on:click=logout class="auth-button">Log Out</button>
        </div>
    }
}