use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::*;
use crate::frienddata::FriendData;


#[component]
pub fn UserRow(user: FriendData) -> impl IntoView {
	  
    view! {
		<div class="friend-row">
            <div class="friend-item">
                <Suspense fallback=|| view! { <Icon class="friend-image" icon=icondata::CgProfile/> }>
                    <img class="friend-image" src={format!("/assets/images/profile/{}.webp", user.user_id)} alt="Profile Photo" />
                </Suspense>
            </div>
			<a class="friend-item" href={format!("user/{}",user.user_id)}>{user.username}</a>
            <p class="friend-item friend-created-date">{user.created_at.format("%m/%d/%Y").to_string()}</p>
		</div>
	}.into_view()
}
