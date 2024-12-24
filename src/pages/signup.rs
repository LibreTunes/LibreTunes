use crate::auth::signup;
use crate::models::User;
use crate::util::state::GlobalState;
use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use leptos::task::spawn_local;
use crate::components::loading::Loading;

#[component]
pub fn Signup() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
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
        let mut new_user = User {
            id: None,
            username: username.get(),
            email: email.get(),
            password: Some(password.get()),
            created_at: None,
            admin: false,
        };
        log!("new user: {:?}", new_user);

        loading.set(true);
        error_msg.set(None);

        let user = GlobalState::logged_in_user();

        spawn_local(async move {
            if let Err(err) = signup(new_user.clone()).await {
                // Handle the error here, e.g., log it or display to the user
                log!("Error signing up: {:?}", err);
                error_msg.set(Some(err.to_string()));

                // Since we're not sure what the state is, manually refetch the user
                user.refetch();
            } else {
                // Manually set the user to the new user, avoiding a refetch
                new_user.password = None;
                user.set(Some(new_user));

                // Redirect to the login page
                log!("Signed up successfully!");
                leptos_router::hooks::use_navigate()("/", Default::default());
                log!("Navigated to home page after signup")
            }

            loading.set(false);
        });
    };

    view! {
        <div class="auth-page-container">
            <div class="signup-container">
                <a class="return" href="/"><Icon icon={icondata::IoReturnUpBackSharp} /></a>
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
                            fallback=move || view!{ <button on:click=toggle_password class="password-visibility"> <Icon icon={icondata::AiEyeInvisibleFilled} /></button> /> }
                        >
                            <button on:click=toggle_password class="password-visibility">
                                <Icon icon={icondata::AiEyeFilled} />
                            </button>
                        </Show>
                    </div>
                    <div class="error-msg">{ move || error_msg.get() }</div>
                    <Show
                        when=move || !loading.get()
                        fallback=move || view!{ <Loading /> }
                    >
                        <input type="submit" value="Sign Up" />
                    </Show>
                    <span class="go-to-login">
                        Already Have an Account? <a href="/login" class="link" >Go to Login</a>
                    </span>
                </form>
            </div>
        </div>
    }
}
