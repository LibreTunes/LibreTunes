use crate::auth::login;
use crate::util::state::GlobalState;
use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use crate::users::UserCredentials;
use crate::components::loading::Loading;

#[component]
pub fn Login() -> impl IntoView {
    let (username_or_email, set_username_or_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let (show_password, set_show_password) = create_signal(false);

    let loading = create_rw_signal(false);
    let error_msg = create_rw_signal(None);

    let toggle_password = move |_| {
        set_show_password.update(|show_password| *show_password = !*show_password);
        log!("showing password");
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let username_or_email1 = username_or_email.get();
        let password1 = password.get();

        spawn_local(async move {
            loading.set(true);
            error_msg.set(None);

            let user_credentials = UserCredentials {
                username_or_email: username_or_email1,
                password: password1
            };

            let user = GlobalState::logged_in_user();
            
            let login_result = login(user_credentials).await;
            if let Err(err) = login_result {
                // Handle the error here, e.g., log it or display to the user
                log!("Error logging in: {:?}", err);
                error_msg.set(Some(err.to_string()));

                // Since we're not sure what the state is, manually refetch the user
                user.refetch();
            } else if let Ok(Some(login_user)) = login_result {
                // Manually set the user to the new user, avoiding a refetch
                user.set(Some(login_user));

                // Redirect to the login page
                log!("Logged in Successfully!");
                leptos_router::use_navigate()("/", Default::default());
                log!("Navigated to home page after login");
            } else if let Ok(None) = login_result {
                log!("Invalid username or password");
                error_msg.set(Some("Invalid username or password".to_string()));

                // User could be already logged in or not, so refetch the user
                user.refetch();
            }

            loading.set(false);
        });
    };

    view! {
        <div class="auth-page-container">
            <div class="login-container">
                <a class="return" href="/"><Icon icon={icondata::IoReturnUpBackSharp} /></a>
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
                        <input class="login-password"  type={move || if show_password() { "text" } else { "password"} } required
                        on:input = move |ev| {
                            set_password(event_target_value(&ev));
                            log!("password changed to: {}", password.get());
                        }
                        />
                        <span>Password</span>
                        <i></i>
                        <Show
                            when=move || {show_password() == false}
                            fallback=move || view!{ <button on:click=toggle_password class="login-password-visibility">
                                                  <Icon icon={icondata::AiEyeInvisibleFilled} />
                                               </button> /> }
                        >
                            <button on:click=toggle_password class="login-password-visibility">
                                <Icon icon={icondata::AiEyeFilled} />
                            </button>

                        </Show>
                    </div>
                    <a href="" class="forgot-pw">Forgot Password?</a>
                    <div class="error-msg" >{ move || error_msg.get() }</div>
                    <Show
                        when=move || !loading.get()
                        fallback=move || view! { <Loading /> }
                    >
                        <input type="submit" value="Login" />
                    </Show>
                    <span class="go-to-signup">
                        New here? <a href="/signup">Create an Account</a>
                    </span>
                </form>
            </div>
        </div>
    }
}
