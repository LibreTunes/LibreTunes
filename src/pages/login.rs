use crate::api::auth::login;
use crate::util::state::GlobalState;
use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use leptos::task::spawn_local;
use crate::api::users::UserCredentials;
use crate::components::loading::Loading;
use crate::components::fancy_input::*;

#[component]
pub fn Login() -> impl IntoView {
    let username_or_email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            loading.set(true);
            error_msg.set(None);

            let user_credentials = UserCredentials {
                username_or_email: username_or_email.get_untracked(),
                password: password.get_untracked(),
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
                user.set(Some(Some(login_user)));

                // Redirect to the login page
                log!("Logged in Successfully!");
                leptos_router::hooks::use_navigate()("/", Default::default());
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
        <section class="bg-white dark:bg-black flex items-center justify-center h-screen">
            <div class="rounded-lg shadow bg-white w-full p-12 max-w-md relative">
                <a class="hover:bg-neutral-400 transition-all duration-500
                    rounded-md absolute left-5 top-5 p-1" href="/">
                    <Icon icon={icondata::IoReturnUpBackSharp} height="1.5rem" width="1.5rem"/>
                </a>
                <h1 class="text-5xl font-bold text-accent text-center p-1">"LibreTunes"</h1>
                <form on:submit=on_submit>
                    <FancyInput label="Username/Email" required=true value=username_or_email />
                    <FancyInput label="Password" password=true required=true value=password />
                    <a class="hover-link my-1">"Forgot Password?"</a>
                    <div
                        class="text-red-800 text-base"
                        style="min-height: calc(var(--text-base--line-height) * var(--text-base));"
                    >
                        { move || error_msg.get() }
                    </div>
                    <Show
                        when=move || !loading.get()
                        fallback=move || view! { <div class="p-3 my-2"> <Loading /> </div> }
                    >
                        <input class="bg-accent rounded-md text-white text-base
                            w-full p-3 my-2 font-semibold cursor-pointer" type="submit" value="Login" />
                    </Show>
                    <span class="text-base text-neutral-500 my-1">
                        "New here?"
                        <a class="hover-link ml-2" href="/signup">"Create an Account"</a>
                    </span>
                </form>
            </div>
        </section>
    }
}
