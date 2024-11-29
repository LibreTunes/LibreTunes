use leptos::leptos_dom::*;
use leptos::*;
use leptos_router::*;
use crate::api::profile::*;
use crate::components::friend_list::*;
use crate::components::loading::Loading;


#[derive(Params, PartialEq)]
struct FriendParams {
    id: i32
}

#[component]
pub fn Friends() -> impl IntoView {
    let params = use_params::<FriendParams>();

    let id = move || {params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .map_err(|e| e.clone())
        })
    };

    let friend_list = create_resource(
        id,
        |value| async move {
            match value {
                Ok(v) => {friends(v).await},
                Err(e) => {Err(ServerFnError::Request(format!("Error getting song data: {}", e).into()))},
            }
        },
    );

    view! {
        <div class="friend-page-container">
            <h1 class="friend-header"> "Friends:" </h1>
            <Transition
                fallback=move || view! {
                    <Loading />
                }
            >
                <ErrorBoundary
                    fallback=|errors| view! {
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { <p>{e.to_string()}</p>})
                            .collect_view()
                        }
                    }
                >
                    {
                        friend_list.get().map(|friend_list| {
                            friend_list.map(|friend_list| {
                                view! {<FriendList friends={friend_list} />}
                            })
                        })
                    }
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

