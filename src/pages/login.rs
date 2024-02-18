use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::IoIcon::*;
use leptos_icons::*;
use crate::auth::login;

#[component]
pub fn Login() -> impl IntoView {
    let (username_or_email, set_username_or_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            if let Err(err) = login(username_or_email.get(), password.get()).await {
                // Handle the error here, e.g., log it or display to the user
                log!("Error logging in: {:?}", err);
            } else {
                // Redirect to the login page
                log!("Logged in Successfully!");
            }
        });
    };

    view! {
        <div class="page-container">
            <div class="login-container">
                <a class="return" href="/"><Icon icon=Icon::from(IoReturnUpBackSharp) /></a>
                <div class="header">
                    <h1>LibreTunes</h1>
                </div>
                <form class="login-form" action="POST" on:submit=on_submit>
                    <div class="input-box">
                        <input class="login-info" type="text" required
                        on:input = move |ev| {
                            set_username_or_email(event_target_value(&ev));
                            log!("username/email changed to: {}", username_or_email.get());
                        }
                        prop:value=username_or_email
                        />
                        <span>Username/Email</span>
                        <i></i>
                    </div>
                    <div class="input-box">
                        <input class="login-password" type="text" required
                        on:input = move |ev| {
                            set_password(event_target_value(&ev));
                            log!("password changed to: {}", password.get());
                        }
                        />
                        <span>Password</span>
                        <i></i>
                    </div>
                    <a href="" class="forgot-pw">Forgot Password?</a>
                    <input type="submit" value="Login" />
                    <span class="go-to-signup">
                        New here? <a href="/signup">Create an Account</a>
                    </span>
                </form>
            </div>
        </div>
    }
}
