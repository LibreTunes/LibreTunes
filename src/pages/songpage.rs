use leptos::*;
use leptos_router::use_params_map;
use leptos_icons::*;
use server_fn::error::NoCustomError;

use crate::components::loading::*;
use crate::components::error::*;
use crate::api::song::*;
use crate::models::Song;
use crate::songs::get_song_by_id;

#[component]
pub fn SongPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <div class="song-container home-component">
            {move || params.with(|params| {
                match params.get("id").map(|id| id.parse::<i32>()) {
                    Some(Ok(id)) => {
                        view! { <SongDetails id /> }.into_view()
                    },
                    Some(Err(e)) => {
                        view! {
                            <Error<String>
                                title="Invalid Song ID"
                                error=e.to_string()
                            />
                        }.into_view()
                    },
                    None => {
                        view! {
                            <Error<String>
                                title="No Song ID"
                                message="You must specify a song ID to view its page."
                            />
                        }.into_view()
                    }
                }
            })}
        </div>
    }
}

#[component]
fn SongDetails(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
    let song_info = create_resource(move || id.get(), move |id| {
        get_song_by_id(id)
    });

    view! {
        <Transition
            fallback=move || view! { <LoadingPage /> }
        >
            {move || song_info.get().map(|song| {
                match song {
                    Ok(Some(song)) => {
                        view! { <SongOverview song /> }.into_view()
                    },
                    Ok(None) => {
                        view! {
                            <Error<String>
                                title="Song Not Found"
                                message=format!("Song with ID {} not found", id.get())
                            />
                        }.into_view()
                    },
                    Err(error) => {
                        view! {
                            <ServerError<NoCustomError>
                                title="Error Fetching Song"
                                error
                            />
                        }.into_view()
                    }
                }
            })}
        </Transition>
    }
}

#[component]
fn SongOverview(song: Song) -> impl IntoView {
    view! {
        <div class="song-header">
            <h1>{song.title}</h1>
            <p>{format!("Artist: {}", song.artist)}</p>
            <p>{format!("Album: {}", song.album.unwrap_or_else(|| "Unknown".to_string()))}</p>
            <p>{format!("Duration: {}", song.duration)}</p>
        </div>
    }
}
