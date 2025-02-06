use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use leptos::task::spawn_local;
use crate::api::auth::logout;
use crate::util::state::GlobalState;

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
    let (dropdown_open, set_dropdown_open) = signal(false);
	let user = GlobalState::logged_in_user();
    
	let open_dropdown = move |_| {
        set_dropdown_open.update(|value| *value = !*value);
    };

    let user_profile_picture = move || {
        user.get().and_then(|user| {
            if let Some(user) = user {
				user.id?;
				Some(format!("/assets/images/profile/{}.webp", user.id.unwrap()))
			} else {
				None
			}
        })
    };

    view! {
        <div class="profile-container">
            <div class="profile-name">
                <Suspense
                    fallback=|| view!{
                        <h1>Not Logged In</h1>
                    }>
					<Show
						when=move || user.get().map(|user| user.is_some()).unwrap_or(false)
						fallback=|| view!{
							<h1>Not Logged In</h1>
						}>
                    	<h1>{move || user.get().map(|user| user.map(|user| user.username))}</h1>
					</Show>
                </Suspense>
            </div>
            <div class="profile-icon" on:click=open_dropdown>
				<Suspense fallback=|| view! { <Icon icon={icondata::CgProfile} width="45" height="45"/> }>
					<Show 
						when=move || user.get().map(|user| user.is_some()).unwrap_or(false)
						fallback=|| view! { <Icon icon={icondata::CgProfile} width="45" height="45"/> }
					>
						<object class="profile-image" data={user_profile_picture} type="image/webp">
							<Icon icon={icondata::CgProfile} width="45" height="45" {..} class="profile-image" />
						</object>
					</Show>
				</Suspense>
            </div>
            <div class="dropdown-container" style={move || if dropdown_open() {"display: flex"} else {"display: none"}}>
				<Suspense
					fallback=|| view!{
						<DropDownNotLoggedIn />
					}>
					<Show
						when=move || user.get().map(|user| user.is_some()).unwrap_or(false)
						fallback=|| view!{
							<DropDownNotLoggedIn />
						}>
						<DropDownLoggedIn />
					</Show>
				</Suspense>
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
				let user = GlobalState::logged_in_user();
				user.refetch();
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
