use crate::auth::signup;
use crate::models::User;
use leptos::leptos_dom::*;
use leptos::*;
use leptos_icons::AiIcon::*;
use leptos_icons::IoIcon::*;
use leptos_icons::*;

#[component]
pub fn Signup() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let (show_password, set_show_password) = create_signal(false);

    let navigate = leptos_router::use_navigate();

    let toggle_password = move |_| {
        set_show_password.update(|show_password| *show_password = !*show_password);
        log!("showing password");
    };

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
                leptos_router::use_navigate()("/", Default::default());
                log!("Navigated to home page after signup")
            }
        });
    };

    view! {
        <div class="auth-page-container">
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
                        <input class="signup-password" type={move || if show_password() { "text" } else { "password"} } required style="width: 90%;"
                        on:input = move |ev| {
                            set_password(event_target_value(&ev));
                            log!("password changed to: {}", password.get());
                        }
                        />
                        <span>Password</span>
                        <i></i>
                        <Show
                            when=move || {show_password() == false}
                            fallback=move || view!{ <button on:click=toggle_password class="password-visibility"> <Icon icon=Icon::from(AiEyeInvisibleFilled) /></button> /> }
                        >
                            <button on:click=toggle_password class="password-visibility">
                                <Icon icon=Icon::from(AiEyeFilled) />
                            </button>
                        </Show>
                    </div>
                    <input type="submit" value="Sign Up"  />
                    <span class="go-to-login">
                        Already Have an Account? <a href="/login" class="link" >Go to Login</a>
                    </span>
                </form>
            </div>
        </div>
    }
}
