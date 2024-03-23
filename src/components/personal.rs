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
    view! {
        <div class="profile-container">
            <div class="profile-icon">
                <Icon icon=Icon::from(CgProfile) />
            </div>
            
        </div>
    }
}
