use leptos::ev;
use leptos::leptos_dom::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    view! {
        <div class="page-container">
            <div class="login-container">
            <div class="header">
                <h1>LibreTunes</h1>
            </div>
                <form class="login-form" action="POST">
                    <div class="input-box">
                        <input class="login-info" type="text" required
                        on:input = move |ev| {
                            set_username(event_target_value(&ev));
                            log!("username changed to: {}", username.get());
                        }
                        prop:value=username
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
                    <p class="forgot-pw">Forgot Password?</p>
                    <input type="submit" value="Login" />
                    <p class="go-to-signup">
                        New here? <span>Create an Account</span>
                    </p>
                </form>
            </div>
        </div>
    }
}
