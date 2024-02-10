use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::IoIcon::*;
use leptos_icons::*;

#[component]
pub fn Signup() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());


    view!{
        <div class="page-container">
            <div class="signup-container">
                <a class="return" href="/"><Icon icon=Icon::from(IoReturnUpBackSharp) /></a>
                <div class="header">
                    <h1>LibreTunes</h1>
                </div>
                <form class="signup-form" action="POST">
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
                    <input type="submit" value="Sign Up" />
                    <span class="go-to-login">
                        Already Have an Account? <a href="/login">Go to Login</a>
                    </span>
                </form>
            </div>
        </div>
    }
}