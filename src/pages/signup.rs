use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::IoIcon::*;
use leptos_icons::*;
use crate::auth::signup;
use crate::models::User;


#[component]
pub fn Signup() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let new_user = User {
            id: None,
            username: username.get(),
            email: email.get(),
            password: Some(password.get()),
            created_at: None,
        };
        log!("new user: {:?}", new_user);
        spawn_local(async move {
            if let Err(err) = signup(new_user).await {
                // Handle the error here, e.g., log it or display to the user
                log!("Error signing up: {:?}", err);
            } else {
                // Redirect to the login page
                log!("Signed up successfully!");
            }
        });
    };

    view!{
        <div class="page-container">
            <div class="signup-container">
                <a class="return" href="/"><Icon icon=Icon::from(IoReturnUpBackSharp) /></a>
                <div class="header">
                    <h1>LibreTunes</h1>
                </div>
                <form class="signup-form" action="POST" on:submit=on_submit>
                    <div class="input-box">
                        <input class="signup-email" type="text" required
                        on:input = move |ev| {
                            set_email(event_target_value(&ev));
                            log!("email changed to: {}", email.get());
                        }
                        prop:value=email
                        />
                        <span>Email</span>
                        <i></i>
                    </div>
                    <div class="input-box">
                        <input class="signup-username" type="text" required
                        on:input = move |ev| {
                            set_username(event_target_value(&ev));
                            log!("username changed to: {}", username.get());
                        }
                        />
                        <span>Username</span>
                        <i></i>
                    </div>
                    <div class="input-box">
                        <input class="signup-password" type="text" required
                        on:input = move |ev| {
                            set_password(event_target_value(&ev));
                            log!("password changed to: {}", password.get());
                        }
                        />
                        <span>Password</span>
                        <i></i>
                    </div>
                    <input type="submit" value="Sign Up"  />
                    <span class="go-to-login">
                        Already Have an Account? <a href="/login">Go to Login</a>
                    </span>
                </form>
            </div>
        </div>
    }
}