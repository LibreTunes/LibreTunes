use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use leptos::task::spawn_local;
use leptos::html::Div;
use leptos_use::on_click_outside_with_options;
use leptos_use::OnClickOutsideOptions;
use crate::api::auth::logout;
use crate::util::state::GlobalState;

#[component]
pub fn Personal() -> impl IntoView {
    view! {
        <div class="home-card">
            <Profile />
        </div>
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let dropdown_open = RwSignal::new(false);
	let user = GlobalState::logged_in_user();

	let toggle_dropdown = move |_| dropdown_open.set(!dropdown_open.get());

    let profile_photo = NodeRef::<Div>::new();
    let dropdown = NodeRef::<Div>::new();
    let _ = on_click_outside_with_options(dropdown, move |_| dropdown_open.set(false),
        OnClickOutsideOptions::default().ignore(profile_photo)
    );

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
        <div class="flex w-50 relative">
            <div class="text-lg self-center">
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
            <div class="self-center hover:scale-105 transition-transform cursor-pointer ml-auto"
                on:click=toggle_dropdown node_ref=profile_photo>
				<Suspense fallback=|| view! { <Icon icon={icondata::CgProfile} width="45" height="45"/> }>
					<Show 
						when=move || user.get().map(|user| user.is_some()).unwrap_or(false)
						fallback=|| view! { <Icon icon={icondata::CgProfile} width="45" height="45"/> }
					>
						<object class="w-11 h-11 rounded-full pointer-events-none"
                            data={user_profile_picture} type="image/webp">
							<Icon icon={icondata::CgProfile} width="45" height="45" {..} />
						</object>
					</Show>
				</Suspense>
            </div>
            <Show when=dropdown_open >
                <div class="absolute bg-bg-light rounded-lg border-2 border-neutral-700 top-12
                    right-3 p-1 text-right" node_ref=dropdown>
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
            </Show>
        </div>
    }
}
#[component]
pub fn DropDownNotLoggedIn() -> impl IntoView {
    view! {
        <a href="/login"><button class="auth-button">"Log In"</button></a><br/>
        <a href="/signup"><button class="auth-button">"Sign Up"</button></a>
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
        <button on:click=logout class="auth-button">"Log Out"</button>
    }
}
