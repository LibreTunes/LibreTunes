use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use crate::auth::get_user;
use crate::auth::logout;

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
    let (image_error, set_image_error) = create_signal(false);
    let user = create_local_resource(move || dropdown_open.get(), |_| async move { get_user().await });

    let open_dropdown = move |_| {
        set_dropdown_open.update(|value| *value = !*value);
    };

    let user_profile_picture = move || {
        user.get().and_then(|user| {
            user.ok().map(|user| format!("/assets/images/profile/{}.webp", user.id.unwrap()))
        })
    };

    view! {
        <div class="profile-container">
            <div class="profile-name">
                <Show
                    when=move || user.get().map(|user| user.is_ok()).unwrap_or_default()
                    fallback=|| view!{
                        <h1>Not Logged In</h1>
                    }>
                    <h1>{move || user.get().map(|user| user.map(|user| user.username).unwrap_or_default())}</h1>
                </Show>
            </div>
            <div class="profile-icon" on:click=open_dropdown>
                <Show 
                    when=move || user.get().map(|user| user.is_ok()).unwrap_or_default()
                    fallback=|| view! { <Icon icon=icondata::CgProfile width="45" height="45"/> }
                >
                    <Show
                        when=move || !image_error()
                        fallback=|| view! { <Icon icon=icondata::CgProfile width="45" height="45"/> }
                    >
                        <img 
                            src=user_profile_picture() 
                            on:error=move |_| set_image_error.set(true)
                        />
                    </Show>
                </Show>
            </div>
            <div class="dropdown-container" style={move || if dropdown_open() {"display: flex"} else {"display: none"}}>
                <Show
                    when=move || user.get().map(|user| user.is_ok()).unwrap_or_default()
                    fallback=|| view!{
                        <DropDownNotLoggedIn />
                    }>
                    <DropDownLoggedIn />
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
pub fn DropDownLoggedIn() -> impl IntoView {
    
	let logout = move |_| {
        spawn_local(async move {
            let result = logout().await;
            if let Err(err) = result {
                log!("Error logging out: {:?}", err);
            } else {
                log!("Logged out successfully");
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
