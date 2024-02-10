use leptos::leptos_dom::*;
use leptos::*;

#[component]
pub fn Signup() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    view!{
        <div class="page-container">
            <div class="signup-container">
                
            </div>
        </div>
    }
}