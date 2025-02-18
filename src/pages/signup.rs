use crate::api::auth::signup;
use crate::models::backend::User;
use crate::util::state::GlobalState;
use leptos::leptos_dom::*;
use leptos::prelude::*;
use leptos_icons::*;
use leptos::task::spawn_local;
use crate::components::loading::Loading;
use crate::components::fancy_input::*;

#[component]
pub fn Signup() -> impl IntoView {
    let username = RwSignal::new("".to_string());
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let mut new_user = User {
            id: None,
            username: username.get_untracked(),
            email: email.get_untracked(),
            password: Some(password.get_untracked()),
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
                user.set(Some(Some(new_user)));

                // Redirect to the login page
                log!("Signed up successfully!");
                leptos_router::hooks::use_navigate()("/", Default::default());
                log!("Navigated to home page after signup");
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
                    <FancyInput label="Email" required=true value=email />
                    <FancyInput label="Username" required=true value=username />
                    <FancyInput label="Password" password=true required=true value=password />
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
                            w-full p-3 my-2 font-semibold cursor-pointer" type="submit" value="Sign Up" />
                    </Show>
                    <span class="text-base text-neutral-500 my-1">
                        "Already have an account?"
                        <a class="hover-link ml-2" href="/login" >"Go to Login"</a>
                    </span>
                </form>
            </div>
        </section>
    }
}
