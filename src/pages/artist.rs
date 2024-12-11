use leptos::*;
use leptos_router::use_params_map;
use leptos_icons::*;
use server_fn::error::NoCustomError;

use crate::components::loading::*;
use crate::components::error::*;

#[component]
pub fn ArtistPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <div class="artist-container home-component">
            {move || params.with(|params| {
                match params.get("id").map(|id| id.parse::<i32>()) {
                    Some(Ok(id)) => {
                        view! { <ArtistProfile id /> }.into_view()
                    },
                    Some(Err(e)) => {
                        view! {
                            <Error<String>
                                title="Invalid Artist ID"
                                error=e.to_string()
                            />
                        }.into_view()
                    },
                    None => {
                        view! {
                            <Error<String>
                                title="No Artist ID"
                                message="You must specify an artist ID to view their page."
                            />
                        }.into_view()
                    }
                }
            })}
        </div>
    }
}

#[component]
fn ArtistProfile(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
    let artist_info = create_resource(move || id.get(), move |id| {
        get_artist_by_id(id)
    });

    view! {
        <Transition
            fallback=move || view! { <LoadingPage /> }
        >
            {move || artist_info.get().map(|artist| {
                match artist {
                    Ok(Some(artist)) => view! { <ArtistDetails artist /> }.into_view(),
                    Ok(None) => view! {
                        <Error<String>
                            title="Artist Not Found"
                            message=format!("Artist with ID {} not found", id.get())
                        />
                    }.into_view(),
                    Err(error) => view! {
                        <ServerError<NoCustomError>
                            title="Error Getting Artist"
                            error
                        />
                    }.into_view(),
                }
            })}
        </Transition>
    }
}
