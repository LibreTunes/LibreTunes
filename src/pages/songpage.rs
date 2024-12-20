use leptos::*;
use leptos_router::use_params_map;
use leptos_icons::*;
use server_fn::error::NoCustomError;

use crate::api::songs;
use crate::components::loading::*;
use crate::components::error::*;
use crate::components::song_list::*;
use crate::api::songs::*;
use crate::songdata::SongData;
use crate::util::state::GlobalState;

use std::rc::Rc;
use std::borrow::Borrow;

const PLAY_BTN_SIZE: &str = "3rem";

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
        <SongPlays id />
        <MySongPlays id />
    }
}

#[component]
fn SongOverview(song: SongData) -> impl IntoView {
    let liked = create_rw_signal(song.like_dislike.map(|ld| ld.0).unwrap_or(false));
    let disliked = create_rw_signal(song.like_dislike.map(|ld| ld.1).unwrap_or(false));

    let playing = create_rw_signal(false);
    let icon = Signal::derive(move || {
        if playing.get() {
            icondata::BsPauseFill
        } else {
            icondata::BsPlayFill
        }
    });

    create_effect(move |_| {
        GlobalState::play_status().with(|status| {
            playing.set(status.queue.front().map(|song| song.id) == Some(song.id) && status.playing);
        });
    });

    let song_rc = Rc::new(song.clone());

    let toggle_play_song = move |_| {
        GlobalState::play_status().update(|status| {
            if status.queue.front().map(|song| song.id) == Some(song_rc.id) {
                status.playing = !status.playing;
            } else {
                if let Some(last_playing) = status.queue.front() {
                    status.queue.push_front(last_playing.clone());
                }

                status.queue.clear();
                status.queue.push_front(<Rc<SongData> as Borrow<SongData>>::borrow(&song_rc).clone());
                status.playing = true;
            }
        });
    };

    view! {
        <div class="song-header">
            <img class="song-image" src=song.image_path />
            <h1>{song.title}</h1>
        </div>
        <div class="song-actions">
            <button on:click=toggle_play_song>
                <Icon class="controlbtn" width=PLAY_BTN_SIZE height=PLAY_BTN_SIZE icon />
            </button>
            <SongLikeDislike song_id=song.id liked disliked /><br/>
        </div>
        <p><SongArtists artists=song.artists /></p>
        <p><SongAlbum album=song.album /></p>
        <p>{format!("Duration: {}:{:02}", song.duration / 60, song.duration % 60)}</p>
    }
}

#[component]
fn SongPlays(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
    let plays = create_resource(move || id.get(), move |id| songs::get_song_plays(id));

    view! {
        <Transition
            fallback=move || view! { <Loading /> }
        >
            {move || plays.get().map(|plays| {
                match plays {
                    Ok(plays) => {
                        view! {
                            <p>{format!("Plays: {}", plays)}</p>
                        }.into_view()
                    },
                    Err(error) => {
                        view! {
                            <ServerError<NoCustomError>
                                title="Error fetching song plays"
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
fn MySongPlays(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
    let plays = create_resource(move || id.get(), move |id| songs::get_my_song_plays(id));

    view! {
        <Transition
            fallback=move || view! { <Loading /> }
        >
            {move || plays.get().map(|plays| {
                match plays {
                    Ok(plays) => {
                        view! {
                            <p>{format!("My Plays: {}", plays)}</p>
                        }.into_view()
                    },
                    Err(error) => {
                        view! {
                            <ServerError<NoCustomError>
                                title="Error fetching my song plays"
                                error
                            />
                        }.into_view()
                    }
                }
            })}
        </Transition>
    }
}
